#!/bin/bash

cargo build -rj4
cp target/release/rbx RBX.app/Contents/MacOS/
codesign --timestamp --deep --force --sign "Apple Development: wdboyes@icloud.com (5SFDQ6365C)" RBX.app