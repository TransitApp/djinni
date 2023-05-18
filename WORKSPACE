workspace(name="snap_djinni")

# WORKSPACE
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "bazel_skylib",
    sha256 = "b8a1527901774180afc798aeb28c4634bdccf19c4d98e7bdd1ce79d1fe9aaad7",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.4.1/bazel-skylib-1.4.1.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.4.1/bazel-skylib-1.4.1.tar.gz",
    ],
)

# See https://github.com/bazelbuild/rules_scala/releases for up to date version information.
rules_scala_version = "c711b4d1f0d1cc386c63ef748c9df14d2f3a187e"
http_archive(
    name = "io_bazel_rules_scala",
    sha256 = "556677f505634da64efc41912d280895e61f5da109d82bdee41cde4120a190a1",
    strip_prefix = "rules_scala-%s" % rules_scala_version,
    type = "zip",
    url = "https://github.com/bazelbuild/rules_scala/archive/%s.zip" % rules_scala_version,
)

load("@io_bazel_rules_scala//:scala_config.bzl", "scala_config")
# Stores Scala version and other configuration
# 2.12 is a default version, other versions can be use by passing them explicitly:
# scala_config(scala_version = "2.11.12")
# Scala 3 requires extras...
#   3.2 should be supported on master. Please note that Scala artifacts for version (3.2.2) are not defined in
#   Rules Scala, they need to be provided by your WORKSPACE. You can use external loader like
#   https://github.com/bazelbuild/rules_jvm_external
scala_config(scala_version = "2.11.12")

load("@io_bazel_rules_scala//scala:scala.bzl", "rules_scala_setup", "rules_scala_toolchain_deps_repositories")

# loads other rules Rules Scala depends on 
rules_scala_setup()

# Loads Maven deps like Scala compiler and standard libs. On production projects you should consider 
# defining a custom deps toolchains to use your project libs instead 
rules_scala_toolchain_deps_repositories(fetch_sources = True)

load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")
rules_proto_dependencies()
rules_proto_toolchains()

load("@io_bazel_rules_scala//scala:toolchains.bzl", "scala_register_toolchains")
scala_register_toolchains()

# optional: setup ScalaTest toolchain and dependencies
load("@io_bazel_rules_scala//testing:scalatest.bzl", "scalatest_repositories", "scalatest_toolchain")
scalatest_repositories()
scalatest_toolchain()

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

RULES_JVM_EXTERNAL_TAG = "4.5"
RULES_JVM_EXTERNAL_SHA = "b17d7388feb9bfa7f2fa09031b32707df529f26c91ab9e5d909eb1676badd9a6"

http_archive(
    name = "rules_jvm_external",
    strip_prefix = "rules_jvm_external-%s" % RULES_JVM_EXTERNAL_TAG,
    sha256 = RULES_JVM_EXTERNAL_SHA,
    url = "https://github.com/bazelbuild/rules_jvm_external/archive/%s.zip" % RULES_JVM_EXTERNAL_TAG,
)

load("@rules_jvm_external//:repositories.bzl", "rules_jvm_external_deps")

rules_jvm_external_deps()

load("@rules_jvm_external//:setup.bzl", "rules_jvm_external_setup")

rules_jvm_external_setup()

load("@rules_jvm_external//:defs.bzl", "maven_install")

maven_install(
    name = "maven_djinni",
    artifacts = [
        "com.google.code.findbugs:jsr305:3.0.2",
        "junit:junit:4.12",
        "org.scala-lang.modules:scala-parser-combinators_2.11:1.0.1",
        "org.yaml:snakeyaml:1.15",
        "com.github.scopt:scopt_2.11:3.2.0",
        "io.reactivex.rxjava2:rxjava:2.2.21"
    ],
    repositories = ["https://maven.google.com", "https://repo1.maven.org/maven2"],
)

# --- Everything below is only used for examples and tests

# android_sdk_repository fails to find build_tools if we don't explicitly set a version.
android_sdk_repository(name = "androidsdk", build_tools_version = "32.0.0")
RULES_ANDROID_NDK_COMMIT= "81ec8b79dc50ee97e336a25724fdbb28e33b8d41"
RULES_ANDROID_NDK_SHA = "b29409496439cdcdb50a8e161c4953ca78a548e16d3ee729a1b5cd719ffdacbf"

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
http_archive(
    name = "rules_android_ndk",
    url = "https://github.com/bazelbuild/rules_android_ndk/archive/%s.zip" % RULES_ANDROID_NDK_COMMIT,
    sha256 = RULES_ANDROID_NDK_SHA,
    strip_prefix = "rules_android_ndk-%s" % RULES_ANDROID_NDK_COMMIT,
)
load("@rules_android_ndk//:rules.bzl", "android_ndk_repository")
android_ndk_repository(name = "androidndk")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

build_bazel_rules_apple_version = "1.0.1"
http_archive(
    name = "build_bazel_rules_apple",
    sha256 = "36072d4f3614d309d6a703da0dfe48684ec4c65a89611aeb9590b45af7a3e592",
    url = "https://github.com/bazelbuild/rules_apple/releases/download/{0}/rules_apple.{0}.tar.gz"
        .format(build_bazel_rules_apple_version),
)

build_bazel_rules_swift_version = "1.0.0"
http_archive(
    name = "build_bazel_rules_swift",
    sha256 = "12057b7aa904467284eee640de5e33853e51d8e31aae50b3fb25d2823d51c6b8",
    url = "https://github.com/bazelbuild/rules_swift/releases/download/{0}/rules_swift.{0}.tar.gz"
        .format(build_bazel_rules_swift_version),
)

build_bazel_apple_support_version = "1.0.0"
http_archive(
    name = "build_bazel_apple_support",
    sha256 = "df317473b5894dd8eb432240d209271ebc83c76bb30c55481374b36ddf1e4fd1",
    url = "https://github.com/bazelbuild/apple_support/releases/download/{0}/apple_support.{0}.tar.gz"
        .format(build_bazel_apple_support_version),
)

rules_kotlin_version = "legacy-1.3.0"
http_archive(
    name = "io_bazel_rules_kotlin",
    url = "https://github.com/bazelbuild/rules_kotlin/archive/{}.zip".format(rules_kotlin_version),
    type = "zip",
    strip_prefix = "rules_kotlin-{}".format(rules_kotlin_version),
    sha256 = "4fd769fb0db5d3c6240df8a9500515775101964eebdf85a3f9f0511130885fde",
)

load("@build_bazel_rules_apple//apple:repositories.bzl", "apple_rules_dependencies")
load("@build_bazel_rules_swift//swift:repositories.bzl", "swift_rules_dependencies")
load("@build_bazel_apple_support//lib:repositories.bzl", "apple_support_dependencies")
load("@io_bazel_rules_kotlin//kotlin:kotlin.bzl", "kotlin_repositories", "kt_register_toolchains")

apple_rules_dependencies()
swift_rules_dependencies()
apple_support_dependencies()

kotlin_repositories()
kt_register_toolchains()

emsdk_version = "3.1.8"

http_archive(
    name = "emsdk",
    strip_prefix = "emsdk-%s/bazel" % emsdk_version,
    type = "zip",
    url = "https://github.com/emscripten-core/emsdk/archive/%s.zip" % emsdk_version,
    sha256 = "7795202a50ab09958d8943f79110de4386ff0f38bf4c97ec1a896885f28fe1cf",
)

load("@emsdk//:deps.bzl", emsdk_deps = "deps")
emsdk_deps()

load("@emsdk//:emscripten_deps.bzl", emsdk_emscripten_deps = "emscripten_deps")
emsdk_emscripten_deps()
