/*
 * This is a Kotlin/JVM ("desktop") library, built from the same Rust core and the
 * same uniffi-generated, JNA-based Kotlin bindings as the Android package at
 * `packages/android/algokit_composer`. It publishes a plain JAR that can be used
 * from any JVM application (desktop, server, etc.) instead of an Android AAR.
 */
import com.vanniktech.maven.publish.JavadocJar
import com.vanniktech.maven.publish.KotlinJvm

plugins {
  alias(libs.plugins.kotlin.jvm)
  alias(libs.plugins.maven.publish)
}

kotlin { jvmToolchain(21) }

dependencies {
  implementation(libs.jna)
  testImplementation(libs.junit)
}

tasks.test {
  // Lets JNA find the native library that build_pkgs copies here for host tests.
  systemProperty("jna.library.path", "src/test/resources")
}

mavenPublishing {
  configure(KotlinJvm(javadocJar = JavadocJar.Empty(), sourcesJar = true))

  coordinates(
    "io.github.algorandecosystem",
    "algokit-composer-jvm",
    System.getenv("VERSION_TAG") ?: "0.0.1-SNAPSHOT"
  )

  pom {
    name.set("AlgoKit Composer (JVM)")
    description.set("Algorand transaction composer for the desktop JVM")
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
        name.set("Joe Polny")
        email.set("joe@algorand.foundation")
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
