// Android library (AAR) wrapping the `seed_vault_ffi` crate: uniffi-generated
// Kotlin bindings in src/main/kotlin plus the per-ABI native library in
// src/main/jniLibs, both produced by `cargo pkg seed_vault aar`.
//
// Kotlin is compiled by AGP 9's built-in Kotlin support, so no separate
// Kotlin plugin is applied.
plugins {
  alias(libs.plugins.android.library)
  alias(libs.plugins.maven.publish)
}

group = "io.github.algorandecosystem"

android {
  namespace = "io.github.algorandecosystem.algokit_seed_vault"
  compileSdk = 36

  defaultConfig {
    minSdk = 28
    testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
  }

  compileOptions {
    sourceCompatibility = JavaVersion.VERSION_21
    targetCompatibility = JavaVersion.VERSION_21
  }

  testOptions {
    unitTests {
      all { it.systemProperty("jna.library.path", "src/test/resources") }
    }
  }
}

dependencies {
  // `api`: the generated bindings expose JNA types to callers.
  api("net.java.dev.jna:jna:5.17.0@aar")
  // Full JNA JAR for host unit tests (includes desktop dispatch libraries).
  testImplementation("net.java.dev.jna:jna:5.17.0")
  testImplementation(libs.junit)
  androidTestImplementation(libs.androidx.junit)
  androidTestImplementation(libs.androidx.test.runner)
}

mavenPublishing {
  coordinates(
    "io.github.algorandecosystem",
    "algokit-seed-vault",
    System.getenv("VERSION_TAG") ?: "0.0.1-SNAPSHOT"
  )

  pom {
    name.set("AlgoKit Seed Vault")
    description.set("Android client for the seed_vault_ta Trusty applet")
    url.set("https://github.com/algorandecosystem/algokit-core")

    licenses {
      license {
        name.set("MIT License")
        url.set("https://opensource.org/licenses/MIT")
      }
    }

    developers {
      developer {
        id.set("algorandecosystem")
        name.set("Algorand Foundation")
        organization.set("Algorand Foundation")
        organizationUrl.set("https://algorand.foundation")
      }
    }

    scm {
      connection.set("scm:git:git://github.com/algorandecosystem/algokit-core.git")
      developerConnection.set("scm:git:ssh://github.com:algorandecosystem/algokit-core.git")
      url.set("https://github.com/algorandecosystem/algokit-core")
    }
  }
}
