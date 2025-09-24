/* Auto-generated rename hints derived from OpenAPI vendor extensions. */

export const WIRE_TO_CANONICAL: Record<string, string> = {
  'application-index': 'app_id',
  'asset-index': 'asset_id',
}

export const CANONICAL_CAMEL_TO_WIRE: Record<string, string> = {
  appId: 'application-index',
  assetId: 'asset-index',
}

export const HAS_RENAME_HINTS = Object.keys(WIRE_TO_CANONICAL).length > 0
