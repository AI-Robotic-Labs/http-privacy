from setuptools import setup, find_packages

setup(
    name="http-privacy",
    version="1.0.3",  # Fixed: Replaced '--' with '-'
    packages=find_packages(),
    install_requires=[
        "Flask==3.1.3",    # Fixed: Properly formatted as a list of strings
        "requests==2.34.2",
        "user_agent==0.1.14",
        "google-generativeai==0.8.6",
        "mcp==1.28.1",
        "qwen-agent==0.0.34",
        "llama-cpp-python==0.3.34",
        "openai==2.45.0",
        "boto3==1.43.55",
        "xai-sdk==1.17.0"
    ],
    author="AI & Robotic Labs",  # Fixed: Closed quotation
    description="HTTP Privacy JS Bindings",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://codeberg.org/Robot_Labs/http-privacy",
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.6",
)
