from setuptools import setup, find_packages
import codecs
import os

root_directory = os.path.abspath(os.path.dirname(__file__))

with codecs.open(os.path.join(root_directory, "README.md"), encoding = "utf-8") as file_handle:
    long_description = "\n" + file_handle.read()
    
VERSION = "0.2.0"
DESCRIPTION = "Hagstrom Electronics key/mouse emulator interface"
LONG_DESCRIPTION = "Abstraction module for Hagstrom Electronics usbtousb key/mouse emulators"

setup(
    name = "hagstrom",
    version = VERSION,
    author = "xiuxiu62 (Justin Cremer)",
    author_email = "<jacremer@live.com>",
    description = DESCRIPTION,
    long_description = LONG_DESCRIPTION,
    long_description_content_type = "text/markdown",
    url = "https://github.com/xiuxiu62/lib-hagstrom",
    packages = find_packages(),
    classifiers = [
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ]
)
