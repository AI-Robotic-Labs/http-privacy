from setuptools import setup, find_packages

setup(
    name="http-privacy",
    version="1.0.0b6",  # Fixed: Replaced '--' with '-'
    packages=find_packages(),
    install_requires=[
        "Flask==3.1.1",    # Fixed: Properly formatted as a list of strings
        "requests==2.32.4",
        "user_agent==0.1.10",
        "google-generativeai==0.8.5",
        "mcp==1.12.2"
        "qwen-agent==0.0.29",
        "llama-cpp-python==0.3.14",
        "openai==1.97.1",
        "boto3==1.39.15",
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
