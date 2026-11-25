# algokit_seed_vault

Android client (AAR) for the `seed_vault_ta` Trusty applet, generated from
`crates/seed_vault_ffi` with uniffi. The wire format comes from
`crates/seed_vault/src/protocol.rs`, the same code the applet is built
from, so the client and applet can't drift apart.

## Building

```sh
cargo pkg seed_vault aar
```

This builds the native library for each ABI into `src/main/jniLibs`,
generates the Kotlin bindings into `src/main/kotlin`, and assembles the AAR.

## Using

```kotlin
dependencies {
    implementation("io.github.algorandecosystem:algokit-seed-vault:<version>")
}
```

Most devices have no seed vault, so check first (off the main thread):

```kotlin
val vault = SeedVaultClient()
when (val status = vault.status()) {
    is SeedVaultStatus.Available -> { /* show seed vault features */ }
    is SeedVaultStatus.NotSupported -> { /* no Trusty on this device */ }
    is SeedVaultStatus.PermissionDenied -> { /* SELinux / device node permissions */ }
    is SeedVaultStatus.ServiceNotFound -> { /* Trusty without seed_vault_ta */ }
    is SeedVaultStatus.IncompatibleVersion -> { /* app and applet need updating together */ }
}
```

Every other call throws a `SeedVaultException` subclass (`NotSupported`,
`PermissionDenied`, `ServiceNotFound`, `Transport`, `Applet(code)`,
`MalformedResponse`, `InvalidArgument`).

## Testing

```sh
./gradlew test                 # host: no-vault behaviour
./gradlew connectedAndroidTest # device/emulator
```

`SeedVaultClientInstrumentedTest` checks that a stock emulator reports the
vault as unavailable. `SeedVaultEndToEndTest` runs create / list / reveal /
import / delete / sign against a real `seed_vault_ta`. It is skipped unless
`status()` is `Available`, e.g. on Cuttlefish with the applet built into
Trusty (`adb root && adb shell setenforce 0` for local bring-up).

To also submit a signed payment to a node, pass a funded account:

```sh
./gradlew connectedAndroidTest \
  -Pandroid.testInstrumentationRunnerArguments.algodUrl=https://testnet-api.algonode.cloud \
  -Pandroid.testInstrumentationRunnerArguments.fundedMnemonic="word1 ... word25" \
  -Pandroid.testInstrumentationRunnerArguments.fundedAlgorithm=ALGO25_ED25519
```
