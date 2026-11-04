# algokit_transact (JVM)

A Kotlin/JVM ("desktop") build of `algokit_transact`, for use in any JVM application
(desktop apps, servers, CLIs, etc.) rather than Android. It's built from the exact
same Rust core and uniffi-generated Kotlin bindings as `packages/android/algokit_transact`
-- the generated Kotlin uses [JNA](https://github.com/java-native-access/jna) to call
into the native library, which works identically on Android and a desktop JVM.

## Building

```sh
cargo pkg transact kotlin
```

This builds the native library for your current host platform, regenerates the
Kotlin bindings, bundles the native library into the JAR's resources (under the
path JNA automatically looks for, e.g. `darwin-aarch64`, `linux-x86-64`,
`win32-x86-64`), and produces a JAR at `build/libs/algokit_transact-*.jar`.

> Note: the JAR only contains the native library for the platform it was built on.
> To publish a single multi-platform JAR, build this on each target OS/arch and
> merge the resulting `src/main/resources/*` platform directories before running
> `./gradlew jar`.

## Testing

```sh
./gradlew test
```
