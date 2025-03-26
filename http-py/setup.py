from setuptools import setup, find_packages

setup(
    name="http-privacy",
    version="1.0.0b1",  # Fixed: Replaced '--' with '-'
    packages=find_packages(),
    install_requires=[
        "Flask==3.1.0",    # Fixed: Properly formatted as a list of strings
        "requests==2.32.3",
        "user_agent==0.1.10",
        "openai==1.68.2",
        "boto3==1.37.21"

    ],
    author="AI & Robotic Labs",  # Fixed: Closed quotation
    description="HTTP Privacy JS Bindings",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/AII-Robotic-Labs/http-privacy",
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.6",
)