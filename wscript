
#
# This file is the default set of rules to compile a Pebble project.
#
# Feel free to customize this to your needs.
#

import os.path
import os
import glob
from waflib import Task, TaskGen
from waflib.TaskGen import extension

top = '.'
out = 'build'

TaskGen.declare_chain(
    name='rustc',
    rule="${RUSTC} -Z no-landing-pads --target arm-none-eabi "  
         "-L ../target/arm-none-eabi/release/deps "
         "${SRC} --emit obj -A dead-code -o ${TGT}",
    ext_in='.rs',
    ext_out='.o',)

def options(ctx):
    ctx.load('pebble_sdk')

def configure(ctx):
    ctx.load('pebble_sdk')
    os.system('cp arm-none-eabi.json build/')

def build(ctx):

    ctx.load('pebble_sdk')

    build_worker = os.path.exists('worker_src')
    binaries = []


    for p in ctx.env.TARGET_PLATFORMS:
        ctx.set_env(ctx.all_envs[p])
        ctx.set_group(ctx.env.PLATFORM_NAME)
        app_elf='{}/pebble-app.elf'.format(ctx.env.BUILD_DIR)

        ctx.env.RUSTC = 'rustc'

        #Wish this could be in configure, but LINKFLAGS gets reset between aplite & basalt
        os.chdir('build')
        ctx.env.LINKFLAGS += glob.glob('../target/arm-none-eabi/release/deps/*.o')
        os.chdir('..')
        
        ctx.pbl_program(source=ctx.path.ant_glob('src/(*.c|main.rs)'),
        target=app_elf)

        if build_worker:
            worker_elf='{}/pebble-worker.elf'.format(ctx.env.BUILD_DIR)
            binaries.append({'platform': p, 'app_elf': app_elf, 'worker_elf': worker_elf})
            ctx.pbl_worker(source=ctx.path.ant_glob('worker_src/**/*.c'),
            target=worker_elf)
        else:
            binaries.append({'platform': p, 'app_elf': app_elf})

    ctx.set_group('bundle')
    ctx.pbl_bundle(binaries=binaries, js=ctx.path.ant_glob('src/js/**/*.js'))
