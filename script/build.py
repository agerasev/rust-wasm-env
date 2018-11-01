#!/usr/bin/python3

import os
import sys
from subprocess import call
import shutil

def lcall(args):
    print("[info] " + " ".join("%s" % a for a in args))
    call(args)

def lcp(src, dst):
    print("[info] copy %s %s" % (src, dst))
    shutil.copyfile(src, dst)

def lrm(fn):
    print("[info] remove %s" % fn)
    os.remove(fn)

if "--help" in sys.argv:
    print("usage: python3 %s [--release] [--gc / --no-gc]" % sys.argv[0])
    exit()

cargo = "cargo"
target = "wasm32-unknown-unknown"
release = "--release" in sys.argv
if release:
    gc = "--no-gc" not in sys.argv
else:
    gc = "--gc" in sys.argv

buildcmd = [cargo, "build"]
if release:
    buildcmd.append("--release")
buildcmd.append("--target=%s" % target)
lcall(buildcmd)

resdir = "wasm/res/"
templatedir = "wasm/template/"
builddir = "target/%s/%s/" % (target, "release" if release else "debug")

lcp(templatedir + "index.html", "index.html")
if gc:
    wasmgc = "wasm-gc"
    lcp(builddir + "main.wasm", "big.wasm")
    try:
        lcall([wasmgc, "big.wasm", "main.wasm"])
    except FileNotFoundError as e:
        if e.filename == wasmgc:
            print("[warn] 'wasm-gc' not found, try to install")
            lcall([cargo, "install", wasmgc])
            lcall([wasmgc, "big.wasm", "main.wasm"])
        else:
            raise
    lrm("big.wasm")
else:
    lcp(builddir + "main.wasm", "main.wasm")

print("[ok] done")
