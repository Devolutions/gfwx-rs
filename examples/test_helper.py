#!/usr/bin/env python
# *-* encoding: utf-8 *-*
"""
Script to automate testing og the library using test_app
and compare from examples. It runs test_app on all images
in directory and then compares their content.
"""

import argparse
import filecmp
import os
import shutil
import subprocess
import sys


def get_pictures(directory):
    for root, _, files in os.walk(directory):
        for f in files:
            if os.path.splitext(f)[-1].lower() == '.png':
                yield os.path.join(os.path.relpath(root, directory), f)


def mkdir_if_not_exist(path):
    try:
        os.makedirs(os.path.dirname(path))
    except OSError:
        pass


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('input_dir', type=str)
    parser.add_argument('-q', '--quality', type=int, default=1024)
    parser.add_argument(
        '-f',
        '--filter',
        type=str,
        default='linear',
        choices=['linear', 'cubic'])
    parser.add_argument(
        '-e',
        '--encoder',
        type=str,
        default='contextual',
        choices=['fast', 'turbo', 'contextual'])
    args = parser.parse_args()
    out_dir = os.path.join(os.getcwd(), 'gfwx_out')
    err_dir = os.path.join(os.getcwd(), 'gfwx_fails')

    test_failed = False
    for f in get_pictures(args.input_dir):
        gfwx_path = os.path.join(out_dir, '{}.gfwx'.format(
            os.path.splitext(f)[0]))
        mkdir_if_not_exist(gfwx_path)
        png_path = os.path.join(out_dir, '{}.png'.format(
            os.path.splitext(f)[0]))
        mkdir_if_not_exist(png_path)
        orig_gfwx_path = os.path.join(
            out_dir, '{}.orig.gfwx'.format(os.path.splitext(f)[0]))
        mkdir_if_not_exist(orig_gfwx_path)

        try:
            gfwx_args = [
                os.path.join(os.getcwd(), 'test_app'),
                os.path.join(args.input_dir, f),
                gfwx_path,
                png_path,
                '--quality',
                str(args.quality),
                '--filter',
                args.filter,
                '--encoder',
                args.encoder,
            ]
            if args.quality < 1024:
                gfwx_args.extend(['--intent', 'bgr'])
            subprocess.check_output(gfwx_args)
        except subprocess.CalledProcessError:
            print('{}: failed to compress the image'.format(f))
            err_path = os.path.join(err_dir, f)
            mkdir_if_not_exist(err_path)
            shutil.copy(os.path.join(os.getcwd(), args.input_dir, f), err_path)
            continue

        if args.quality == 1024:
            try:
                subprocess.check_output([
                    os.path.join(os.getcwd(), 'compare'),
                    os.path.join(args.input_dir, f), png_path
                ])
            except subprocess.CalledProcessError:
                print('{}: lossless decompression produced different image'.
                      format(f))
                err_path = os.path.join(err_dir, f)
                mkdir_if_not_exist(err_path)
                shutil.copy(
                    os.path.join(os.getcwd(), args.input_dir, f), err_path)
                continue
        else:
            try:
                subprocess.check_output([
                    os.path.join(os.getcwd(), 'reference_test_app'),
                    os.path.join(args.input_dir, f),
                    orig_gfwx_path,
                    str(args.quality),
                    args.filter,
                    args.encoder,
                ])
            except subprocess.CalledProcessError:
                print('{}: reference failed to compress the image'.format(f))
                err_path = os.path.join(err_dir, f)
                mkdir_if_not_exist(err_path)
                shutil.copy(
                    os.path.join(os.getcwd(), args.input_dir, f), err_path)
                continue

            if not filecmp.cmp(orig_gfwx_path, gfwx_path, shallow=False):
                print('{}: reference implementation produced different output'.format(f))
                err_path = os.path.join(err_dir, f)
                mkdir_if_not_exist(err_path)
                shutil.copy(
                    os.path.join(os.getcwd(), args.input_dir, f), err_path)
                continue

        print('{}: OK'.format(f))

    if test_failed:
        return 1
    else:
        return 0


if __name__ == '__main__':
    sys.exit(main())
