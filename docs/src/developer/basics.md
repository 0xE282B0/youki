# Basics

This section has the general information and resources needed to work with any part of youki. As youki is written in Rust, you should know some basic Rust before. If you don't yet, some good resources for that can be found on the Rust's [official site](https://www.rust-lang.org/learn).

## Youki

Youki is a low level container runtime, which deals with the creation and management of Linux containers. Some of other such low-level runtimes are [runc](https://github.com/opencontainers/runc) and [crun](https://github.com/containers/crun). These are usually used by a higher-level runtime such as Docker or Podman to actually create and manage containers, where the higher level runtime provides a much easier interface for users.

Before you start working on developing youki, you should go through [the User documentation](../user/introduction) as it specifies the requirements and setup for running youki. For developing youki, you will need to install the dependencies and clone the repo, as specified in the [Basic Setup](../user/basic_setup.md) and [Basic Usage](../user/basic_usage.md) sections.

## Resources

#### OCI

Open containers initiative is project, which provides a standardization and standardized specification for operating-system-level virtualization. That way components that confirm to the specification provided by OCI spec, can interoperate with each other easily, and developing of new applications becomes easier. For example youki can be used inplace of runc in Docker, as all three : Docker, runc and youki are OCI compliant, and have a standard interface.

Their main GitHub page is at [https://github.com/opencontainers](https://github.com/opencontainers), and more information about the runtime specifications can be found at [https://github.com/opencontainers/runtime-spec/blob/master/runtime.md](https://github.com/opencontainers/runtime-spec/blob/master/runtime.md).

As youki needs to deal with a lot of low level programming interfaces of Linux Kernel, another good place know is the online man pages project, which can be found at [https://man7.org/](https://man7.org/). Man pages provide detailed information about the programming interfaces of various features of Linux Kernel. You can simply search `man <feature-name>` using a search engine, or you can search at the site itself, at [https://man7.org/linux/man-pages/index.html](https://man7.org/linux/man-pages/index.html). These can be very helpful to know about the behavior and usage of and reasoning behind various kernel features used throughout youki.

Happy developing!!!
