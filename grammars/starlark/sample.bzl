"""A sample."""

load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library")

LIMIT = 10

def _sample_impl(ctx):
    out = ctx.actions.declare_file(ctx.label.name + ".txt")
    ctx.actions.write(out, "limit: %d\n" % LIMIT)
    return [DefaultInfo(files = depset([out]))]

sample = rule(
    implementation = _sample_impl,
    attrs = {
        "srcs": attr.label_list(allow_files = True),
        "deps": attr.label_list(),
    },
)

def largest(items):
    best = None
    for item in items:
        if best == None or item > best:
            best = item
    return best

cc_library(
    name = "shape",
    srcs = ["shape.c"],
    hdrs = ["shape.h"],
    visibility = ["//visibility:public"],
)

cc_binary(
    name = "sample",
    srcs = glob(["src/*.c"], exclude = ["src/*_test.c"]),
    copts = ["-O2"] + select({
        "//conditions:default": [],
    }),
    deps = [":shape"],
)

# A comment.
squares = [x * x for x in range(LIMIT) if x % 2 == 0]
names = {"one": 1, "two": 2}
