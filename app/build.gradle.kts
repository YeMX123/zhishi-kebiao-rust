plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}
android {
    namespace = "com.zhishi.kebiao"
    compileSdk = 35
    defaultConfig {
        applicationId = "com.zhishi.kebiao"
        minSdk = 24
        targetSdk = 35
        versionCode = 330
        versionName = "3.3.0"
        ndk {
            abiFilters += listOf("arm64-v8a", "armeabi-v7a")
        }
    }
    signingConfigs {
        create("release") {
            storeFile = file("signing/zhishi_kebiao.p12")
            storePassword = "zhishi_kebiao"
            keyAlias = "zhishi"
            keyPassword = "zhishi_kebiao"
            storeType = "PKCS12"
        }
    }
    buildTypes {
        release {
            isMinifyEnabled = false
            signingConfig = signingConfigs.getByName("release")
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions { jvmTarget = "17" }
}
dependencies {
    implementation("androidx.core:core-ktx:1.13.1")
    implementation("androidx.appcompat:appcompat:1.7.0")
    implementation("androidx.activity:activity-ktx:1.9.1")
}
