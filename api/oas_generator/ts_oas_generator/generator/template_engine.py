"""
TypeScript Template Engine for OpenAPI Client Generation (Phase 2)

Generates runtime plus models/services when a spec is provided.
"""

from __future__ import annotations

import re
from pathlib import Path
from typing import Any

from jinja2 import Environment, FileSystemLoader, select_autoescape

from rust_oas_generator.parser.oas_parser import OASParser
from ts_oas_generator.generator.filters import FILTERS, ts_camel_case, ts_pascal_case, ts_type

_HTTP_METHODS = {"get", "post", "put", "delete", "patch", "head", "options"}

# Compact alias for the build context tuple to keep lines within ruff's E501 limit
BuildContext = tuple[
    dict[str, list[dict[str, Any]]],
    set[str],
    dict[str, Any],
]


class TsTemplateEngine:
    """Template engine for generating TypeScript client code."""

    def __init__(self, template_dir: Path | None = None) -> None:
        if template_dir is None:
            current_dir = Path(__file__).parent
            template_dir = current_dir.parent / "templates"

        self.template_dir = Path(template_dir)
        self.env = Environment(
            loader=FileSystemLoader(str(self.template_dir)),
            autoescape=select_autoescape(["html", "xml"]),
            trim_blocks=True,
            lstrip_blocks=True,
        )

        self._register_filters()
        self._register_globals()

    def _register_filters(self) -> None:
        self.env.filters.update(FILTERS)

    def _register_globals(self) -> None:
        self.env.globals.update({})

    def render_template(self, template_name: str, context: dict[str, Any]) -> str:
        template = self.env.get_template(template_name)
        return template.render(**context)


class TsCodeGenerator:
    """Generates TypeScript runtime and code from templates."""

    def __init__(self, template_dir: Path | None = None) -> None:
        self.template_engine = TsTemplateEngine(template_dir)

    def generate_runtime(
        self,
        output_dir: Path,
        package_name: str = "api_ts_client",
        *,
        custom_description: str | None = None,
    ) -> dict[Path, str]:
        """Generate runtime files under the provided output directory."""
        output_dir = Path(output_dir)
        src_core = output_dir / "src" / "core"
        base_name = ts_pascal_case(package_name)
        base_core = base_name[:-6] if base_name.lower().endswith("client") else base_name
        client_class_name = f"{base_core}Client"
        service_class_name = f"{base_core}Api"

        context = {
            "package_name": package_name,
            "custom_description": custom_description,
            "client_class_name": client_class_name,
            "service_class_name": service_class_name,
        }

        files: dict[Path, str] = {}
        # Core runtime files
        files[src_core / "ClientConfig.ts"] = self.template_engine.render_template(
            "base/src/core/ClientConfig.ts.j2", context
        )
        files[src_core / "BaseHttpRequest.ts"] = self.template_engine.render_template(
            "base/src/core/BaseHttpRequest.ts.j2", context
        )
        files[src_core / "FetchHttpRequest.ts"] = self.template_engine.render_template(
            "base/src/core/FetchHttpRequest.ts.j2", context
        )
        files[src_core / "ApiError.ts"] = self.template_engine.render_template("base/src/core/ApiError.ts.j2", context)
        files[src_core / "request.ts"] = self.template_engine.render_template("base/src/core/request.ts.j2", context)
        files[src_core / "CancelablePromise.ts"] = self.template_engine.render_template(
            "base/src/core/CancelablePromise.ts.j2", context
        )
        files[src_core / "json.ts"] = self.template_engine.render_template("base/src/core/json.ts.j2", context)
        files[src_core / "msgpack.ts"] = self.template_engine.render_template("base/src/core/msgpack.ts.j2", context)
        files[src_core / "casing.ts"] = self.template_engine.render_template("base/src/core/casing.ts.j2", context)

        # Index barrel (runtime-only)
        files[output_dir / "src" / "index.ts"] = self.template_engine.render_template("base/src/index.ts.j2", context)

        # Project files
        files[output_dir / "package.json"] = self.template_engine.render_template("base/package.json.j2", context)
        files[output_dir / "tsconfig.json"] = self.template_engine.render_template("base/tsconfig.json.j2", context)
        files[output_dir / "README.md"] = self.template_engine.render_template("base/README.md.j2", context)
        files[output_dir / ".prettierignore"] = self.template_engine.render_template("base/.prettierignore.j2", context)

        return files

    def _build_services_context(self, spec: dict[str, Any]) -> BuildContext:  # noqa: C901, PLR0912, PLR0915
        """Build a per-tag mapping of operations for service generation.

        Returns (ops_by_tag, tags_set)
        """
        paths = spec.get("paths", {})
        components = spec.get("components", {})
        schemas = components.get("schemas", {})
        model_names = {ts_pascal_case(name) for name in schemas}
        builtin_types = {"Uint8Array", "void", "never", "string", "number", "bigint", "boolean"}

        ops_by_tag: dict[str, list[dict[str, Any]]] = {}
        tags_set: set[str] = set()
        synthetic_models: dict[str, Any] = {}

        token_re = re.compile(r"\b[A-Z][A-Za-z0-9_]*\b")

        def collect_model_types(type_str: str) -> set[str]:
            found: set[str] = set()
            for tok in token_re.findall(type_str or ""):
                if tok in model_names and tok not in builtin_types:
                    found.add(tok)
            return found

        for path, path_item in paths.items():
            if not isinstance(path_item, dict):
                continue
            for method, op in path_item.items():
                if method.lower() not in _HTTP_METHODS:
                    continue
                if not isinstance(op, dict):
                    continue
                operation_id = op.get("operationId")
                if not operation_id:
                    # Fallback to verb + normalized path
                    operation_id = ts_camel_case(f"{method.lower()}_{path}")

                tags = op.get("tags", []) or ["default"]
                parameters = op.get("parameters", [])
                # Gather params from path level too
                if isinstance(path_item.get("parameters"), list):
                    parameters = [*path_item.get("parameters", []), *parameters]

                # Prepare parameters
                param_contexts: list[dict[str, Any]] = []
                # Track used variable names per operation to ensure uniqueness
                used_var_names: set[str] = set()
                reserved_words = {
                    "abstract",
                    "any",
                    "as",
                    "boolean",
                    "break",
                    "case",
                    "catch",
                    "class",
                    "const",
                    "continue",
                    "debugger",
                    "default",
                    "delete",
                    "do",
                    "else",
                    "enum",
                    "export",
                    "extends",
                    "false",
                    "finally",
                    "for",
                    "from",
                    "function",
                    "if",
                    "implements",
                    "import",
                    "in",
                    "instanceof",
                    "interface",
                    "let",
                    "new",
                    "null",
                    "number",
                    "package",
                    "private",
                    "protected",
                    "public",
                    "return",
                    "static",
                    "string",
                    "super",
                    "switch",
                    "symbol",
                    "this",
                    "throw",
                    "true",
                    "try",
                    "type",
                    "typeof",
                    "undefined",
                    "var",
                    "void",
                    "while",
                    "with",
                    "yield",
                    "await",
                    "async",
                    "constructor",
                }
                for p in parameters:
                    param = p
                    if "$ref" in param:
                        # Resolve $ref
                        ref_path = param["$ref"].split("/")[1:]
                        node: Any = spec
                        for part in ref_path:
                            node = node.get(part, {})
                        param = node
                    schema = param.get("schema", {}) or {}
                    t = ts_type(schema, schemas)
                    # When a parameter resolves to bigint, accept number | bigint for ergonomics
                    if t == "bigint":
                        t_sig = "number | bigint"
                        stringify_bigint = True
                    else:
                        t_sig = t
                        stringify_bigint = "bigint" in t_sig

                    raw_name = str(param.get("name"))
                    base_var_name = ts_camel_case(raw_name)
                    # Avoid reserved words
                    if base_var_name in reserved_words:
                        base_var_name = f"{base_var_name}_"
                    var_name = base_var_name
                    # Ensure uniqueness by suffixing with numeric counter
                    if var_name in used_var_names:
                        i = 2
                        while f"{base_var_name}{i}" in used_var_names:
                            i += 1
                        var_name = f"{base_var_name}{i}"
                    used_var_names.add(var_name)

                    param_contexts.append(
                        {
                            "name": raw_name,  # original name used in path/query/header keys
                            "varName": var_name,  # sanitized TS identifier for function signature
                            "in": param.get("in", "query"),
                            "required": param.get("required", False) or (param.get("in") == "path"),
                            "tsType": t_sig,
                            "description": param.get("description"),
                            "stringifyBigInt": stringify_bigint,
                        }
                    )

                # Request body handling
                request_body_ctx: dict[str, Any] | None = None
                request_body_supports_msgpack = False
                request_body_supports_json = False
                rb = op.get("requestBody")
                if isinstance(rb, dict):
                    content = rb.get("content", {})

                    # Check what content types are supported
                    if "application/msgpack" in content:
                        request_body_supports_msgpack = True
                    if "application/json" in content:
                        request_body_supports_json = True

                    # Determine the type to use for TypeScript typing
                    # Prefer JSON schema for typing even when msgpack is used at runtime
                    if "application/json" in content:
                        sch = (content["application/json"] or {}).get("schema", {})
                        request_body_ctx = {
                            "mediaType": (
                                "application/msgpack"
                                if request_body_supports_msgpack and not request_body_supports_json
                                else "application/json"
                            ),
                            "tsType": ts_type(sch, schemas),
                            "required": rb.get("required", False),
                            "supportsMsgpack": request_body_supports_msgpack,
                            "supportsJson": request_body_supports_json,
                        }
                    elif "application/msgpack" in content:
                        # msgpack-only endpoint - infer type from msgpack schema if available
                        sch = (content["application/msgpack"] or {}).get("schema", {})
                        request_body_ctx = {
                            "mediaType": "application/msgpack",
                            "tsType": ts_type(sch, schemas) if sch else "any",
                            "required": rb.get("required", False),
                            "supportsMsgpack": True,
                            "supportsJson": False,
                        }
                    elif "application/x-binary" in content or "application/octet-stream" in content:
                        # Raw binary transaction submission etc.
                        mt = "application/x-binary" if "application/x-binary" in content else "application/octet-stream"
                        request_body_ctx = {
                            "mediaType": mt,
                            "tsType": "Uint8Array",
                            "required": rb.get("required", False),
                            "supportsMsgpack": False,
                            "supportsJson": False,
                        }
                    elif "text/plain" in content:
                        sch = (content["text/plain"] or {}).get("schema", {})
                        request_body_ctx = {
                            "mediaType": "text/plain",
                            "tsType": ts_type(sch, schemas),
                            "required": rb.get("required", False),
                            "supportsMsgpack": False,
                            "supportsJson": False,
                        }

                # Responses
                responses = op.get("responses", {}) or {}
                return_types: list[str] = []
                returns_msgpack = False
                supports_json = False
                for status, resp in responses.items():
                    if not str(status).startswith("2"):
                        continue
                    content = (resp or {}).get("content", {})
                    if "application/msgpack" in content:
                        returns_msgpack = True
                    # Prefer JSON schema for typing
                    if "application/json" in content:
                        supports_json = True
                        sch = (content["application/json"] or {}).get("schema", {})
                        if sch:
                            # If inline object schema without $ref, synthesize a named model from operationId
                            if (
                                isinstance(sch, dict)
                                and "$ref" not in sch
                                and (
                                    sch.get("type") == "object" or "properties" in sch or "additionalProperties" in sch
                                )
                            ):
                                model_name = ts_pascal_case(operation_id)
                                if model_name not in model_names:
                                    synthetic_models[model_name] = sch
                                    model_names.add(model_name)
                                return_types.append(model_name)
                            else:
                                return_types.append(ts_type(sch, schemas))
                    elif content:
                        # Fallback: take first content schema
                        first_ct = next(iter(content.values()))
                        sch = (first_ct or {}).get("schema", {})
                        if sch:
                            return_types.append(ts_type(sch, schemas))

                response_ts_type = "never"
                if supports_json and return_types:
                    # Prefer JSON typing when available
                    uniq: list[str] = []
                    for t in return_types:
                        if t not in uniq:
                            uniq.append(t)
                    response_ts_type = " | ".join(uniq)
                elif returns_msgpack:
                    response_ts_type = "Uint8Array"
                else:
                    response_ts_type = "void"

                # Build signature with required path params first, then params object and request options
                path_params = [p for p in param_contexts if p.get("in") == "path"]
                other_params = [p for p in param_contexts if p.get("in") in {"query", "header"}]

                # detect optional format param for content negotiation
                has_format_param = False
                format_var_name = None
                for qp in other_params:
                    if qp.get("in") == "query" and qp.get("name") == "format":
                        has_format_param = True
                        format_var_name = qp.get("varName")

                sig_parts: list[str] = [f"{p['varName']}: {p['tsType']}" for p in path_params]
                # params bag and request options
                param_sigs: list[str] = [
                    f"{p['varName']}{'' if p['required'] else '?'}: {p['tsType']}" for p in other_params
                ]
                if request_body_ctx:
                    body_required = bool((request_body_ctx or {}).get("required"))
                    body_type = str(request_body_ctx["tsType"])
                    body_param_sig = f"body{'' if body_required else '?'}: {body_type}"
                    param_sigs.append(body_param_sig)
                sig_parts.append("params?: { " + ", ".join(param_sigs) + " }")
                sig_parts.append("requestOptions?: ApiRequestOptions")

                # Import types from embedded types
                import_types: set[str] = set()
                import_types |= collect_model_types(response_ts_type)
                if request_body_ctx:
                    import_types |= collect_model_types(str(request_body_ctx.get("tsType") or ""))
                # Also from parameter types (e.g., enums not needed as imports)
                for p in param_contexts:
                    import_types |= collect_model_types(p.get("tsType") or "")

                op_ctx = {
                    "operationId": operation_id,
                    "method": method.upper(),
                    "path": path,
                    "parameters": param_contexts,
                    "pathParameters": path_params,
                    "otherParameters": other_params,
                    "requestBody": request_body_ctx,
                    "responseTsType": response_ts_type,
                    "acceptsMsgpack": returns_msgpack,
                    "returnsMsgpack": returns_msgpack,
                    "supportsJson": supports_json,
                    "requestBodySupportsMsgpack": request_body_supports_msgpack,
                    "requestBodySupportsJson": request_body_supports_json,
                    "hasFormatParam": has_format_param,
                    "formatVarName": format_var_name,
                    "signature": ", ".join(sig_parts),
                    "importTypes": sorted(import_types),
                }

                for tag in tags:
                    tags_set.add(tag)
                    ops_by_tag.setdefault(tag, []).append(op_ctx)

        # Stable sort operations per tag by operationId
        for _tag, ops in ops_by_tag.items():
            ops.sort(key=lambda o: o.get("operationId") or "")

        return ops_by_tag, tags_set, synthetic_models

    def generate_full(
        self,
        spec_path: Path,
        output_dir: Path,
        package_name: str,
        *,
        custom_description: str | None = None,
    ) -> dict[Path, str]:
        """Generate runtime + models + apis from a spec."""
        parser = OASParser()
        parser.parse_file(spec_path)

        files = self.generate_runtime(output_dir, package_name, custom_description=custom_description)

        spec_dict: dict[str, Any] = parser.spec_data or {}
        components = spec_dict.get("components", {})
        raw_schemas: dict[str, Any] = components.get("schemas", {})

        # Build operations and synthetic models
        ops_by_tag, tags_set, synthetic_models = self._build_services_context(spec_dict)

        # Merge component schemas with synthetic models
        combined_schemas: dict[str, Any] = dict(raw_schemas)
        for m_name, m_schema in synthetic_models.items():
            if m_name not in combined_schemas:
                combined_schemas[m_name] = m_schema

        # Models
        models_dir = Path(output_dir) / "src" / "models"
        for name, schema in combined_schemas.items():
            content = self.template_engine.render_template(
                "models/model.ts.j2",
                {"schema_name": name, "schema": schema, "schemas": combined_schemas},
            )
            files[models_dir / f"{name.lower()}.ts"] = content

        # Models barrel
        files[models_dir / "index.ts"] = self.template_engine.render_template(
            "models/index.ts.j2",
            {"schemas": combined_schemas},
        )

        # Consolidated single service with all operations
        apis_dir = Path(output_dir) / "src" / "apis"
        all_ops: list[dict[str, Any]] = []
        seen_keys: set[tuple[str, str]] = set()
        for tag in sorted(tags_set):
            for op in ops_by_tag.get(tag, []):
                key = (str(op.get("method")), str(op.get("path")))
                if key in seen_keys:
                    continue
                seen_keys.add(key)
                all_ops.append(op)
        # Stable sort by operationId
        all_ops.sort(key=lambda o: o.get("operationId") or "")

        # Aggregate imports
        import_types: set[str] = set()
        for op in all_ops:
            for t in op.get("importTypes", []):
                import_types.add(t)

        # Determine class names from package name (e.g., algod_client -> AlgodClient, AlgodApi)
        base_name = ts_pascal_case(package_name)
        base_core = base_name[:-6] if base_name.lower().endswith("client") else base_name
        client_class_name = f"{base_core}Client"
        service_class_name = f"{base_core}Api"

        svc_content = self.template_engine.render_template(
            "apis/service.ts.j2",
            {
                "tag_name": "api",
                "operations": all_ops,
                "import_types": sorted(import_types),
                "service_class_name": service_class_name,
            },
        )
        files[apis_dir / "api.service.ts"] = svc_content

        files[apis_dir / "index.ts"] = self.template_engine.render_template(
            "apis/index.ts.j2",
            {"service_class_name": service_class_name},
        )

        # Client (single service)
        files[Path(output_dir) / "src" / "client.ts"] = self.template_engine.render_template(
            "client.ts.j2",
            {"service_class_name": service_class_name, "client_class_name": client_class_name},
        )

        # Replace index with full barrel
        files[Path(output_dir) / "src" / "index.ts"] = self.template_engine.render_template(
            "full/src/index.ts.j2",
            {},
        )

        # Note: Generated smoke tests removed; prefer manual integration tests maintained by developers

        return files
