[![Build Status](https://travis-ci.org/cecton/aws-zip.svg?branch=master)](https://travis-ci.org/cecton/aws-zip)
[![Latest Version](https://img.shields.io/crates/v/aws-zip.svg)](https://crates.io/crates/aws-zip)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](http://opensource.org/licenses/MIT)
[![LOC](https://tokei.rs/b1/github/cecton/aws-zip)](https://github.com/cecton/aws-zip)
[![Dependency Status](https://deps.rs/repo/github/cecton/aws-zip/status.svg)](https://deps.rs/repo/github/cecton/aws-zip)

aws-zip
=======

Exactly similar to the `zip -r` command but with a few quirks:

1.  **It doesn't store the modification time:**

    Zipping the same file with the same content but different modification time
    will produce always the same result. This is useful if you plan to deploy
    ZIPs on AWS Lambda for example.

2.  **It allows you to compress in bzip2 instead of deflate:**

    This is also useful if you want to reduce your ZIP to upload on AWS.

3.  **Statically linked:**

    Just curl the file in your CI's build and it will work instantly.

4.  **It preserve executable permissions:**

     *  Directories: 755
     *  Executable files: 755
     *  Not executable files: 644
