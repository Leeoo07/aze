# mycroft

A time tracking tool heavenly inspired by watson.

## Quickstart

### Installation

Simply download the binary from the [Release](https://github.com/kreemer/mycroft/releases/) page.

### Usage

Start tracking your time analogous to watson:

```console
$ mycroft start universe-domination +dog
```

With this command you started a new activity `universe-domination` with the tag `dog`.

You can stop your tracked project with:

```console
$ mycroft stop
```

You can view your tracked projects with:

```console
$ mycroft log
```

Please read the additional documentation with:

```console
$ mycroft help
```

## Internals

The tracked projects will be tracked in a sqlite database.

