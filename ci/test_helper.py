#!/usr/bin/env python
# -*- encoding: utf-8 -*-
"""
Script to automate testing of the library using binaries
from examples directory and reference application
"""

import argparse
import filecmp
import os
import shutil
import subprocess
import sys

OUT_DIR = os.path.join(os.getcwd(), 'gfwx_out')


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


def get_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('input_dir', type=str)
    return parser.parse_args()


class Params(object):
    def __init__(self, name, quality, gfwx_filter, encoder, downsampling):
        self.name = name
        self.quality = quality
        self.filter = gfwx_filter
        self.encoder = encoder
        self.downsampling = downsampling


def compress(image, params):
    output = os.path.join(
        OUT_DIR, '{}.gfwx'.format(
            os.path.splitext(os.path.basename(image))[0]))
    mkdir_if_not_exist(output)
    gfwx_params = [
        os.path.join(os.getcwd(), 'compress'),
        image,
        output,
        '--quality',
        str(params.quality),
        '--filter',
        params.filter,
        '--encoder',
        params.encoder,
        '--intent',
        'bgr',
    ]
    try:
        subprocess.check_output(gfwx_params)
        return (True, )
    except subprocess.CalledProcessError:
        return (False, 'failed to compress image')


def decompress(image, params):
    gfwx = os.path.join(
        OUT_DIR, '{}.gfwx'.format(
            os.path.splitext(os.path.basename(image))[0]))
    mkdir_if_not_exist(gfwx)
    output = os.path.join(OUT_DIR, os.path.basename(image))
    mkdir_if_not_exist(output)
    gfwx_params = [
        os.path.join(os.getcwd(), 'decompress'),
        gfwx,
        output,
        '--downsampling',
        str(params.downsampling),
    ]
    try:
        subprocess.check_output(gfwx_params)
        return (True, )
    except subprocess.CalledProcessError:
        return (False, 'failed to decompress the image')


def compare_with_reference(image, params):
    orig_gfwx_path = os.path.join(
        OUT_DIR, '{}.orig_gfwx'.format(
            os.path.splitext(os.path.basename(image))[0]))
    mkdir_if_not_exist(orig_gfwx_path)

    try:
        subprocess.check_output([
            os.path.join(os.getcwd(), 'reference_test_app'),
            image,
            orig_gfwx_path,
            str(params.quality),
            params.filter,
            params.encoder,
        ])
    except subprocess.CalledProcessError:
        return (False, "reference failed to compress the image")

    gfwx_path = os.path.join(
        OUT_DIR, '{}.gfwx'.format(
            os.path.splitext(os.path.basename(image))[0]))
    same = filecmp.cmp(orig_gfwx_path, gfwx_path, shallow=False)
    if same:
        return (True, )
    else:
        return (False, "reference produced different output")


def compare_with_decompressed(image, params):
    decompressed = os.path.join(OUT_DIR, os.path.basename(image))
    try:
        subprocess.check_output([
            os.path.join(os.getcwd(), 'compare'),
            image,
            decompressed,
        ])
        return (True, )
    except subprocess.CalledProcessError:
        return (False, "compressed and decompressed images are different")


def test(test_case, image, params):
    result = test_case(image, params)
    if not result[0]:
        print('{}: {}'.format(os.path.basename(image), result[1]))
        err_path = os.path.join(os.getcwd(), 'gfwx_fails',
                                os.path.basename(os.path.basename(image)))
        mkdir_if_not_exist(err_path)
        shutil.copy(image, err_path)
        return False
    else:
        return True


def main():
    tests = [
        (Params('lossless_linear_turbo', 1024, 'linear', 'turbo', 0), [
            compress, decompress, compare_with_reference,
            compare_with_decompressed
        ]),
        (Params('losy_cubic_fast', 124, 'cubic', 'fast', 0),
         [compress, decompress, compare_with_reference]),
        (Params('losy_cubic_context', 64, 'cubic', 'contextual', 0),
         [compress, decompress, compare_with_reference]),
    ]
    input_dir = get_args().input_dir

    test_failed = False
    for params, test_cases in tests:
        print('==== Case: {}'.format(params.name))
        for image in get_pictures(input_dir):
            for case in test_cases:
                if test(case, os.path.join(input_dir, image), params):
                    test_failed = True
                    continue

            print('\t{}: OK'.format(image))

    if test_failed:
        return 1
    else:
        return 0


if __name__ == '__main__':
    sys.exit(main())
