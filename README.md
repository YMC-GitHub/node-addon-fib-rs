# `@yors/fib-rs`

a fib function for node in rust with napi-rs/package/template

[![CI](https://github.com/YMC-GitHub/node-addon-fib-rs/actions/workflows/CI.yml/badge.svg)](https://github.com/YMC-GitHub/node-addon-fib-rs/actions/workflows/CI.yml)

[![Lint](https://github.com/YMC-GitHub/node-addon-fib-rs/actions/workflows/lint.yml/badge.svg)](https://github.com/YMC-GitHub/node-addon-fib-rs/actions/workflows/lint.yml)

> Template project for writing node packages with napi-rs.

# Usage

1. Click **Use this template**.
2. **Clone** your project.
3. Run `yarn install` to install dependencies.
4. Run `npx napi rename -n [name]` command under the project folder to rename your package.

## Install this test package

```bash
yarn add @yors/fib-rs
```

## demo

```ts
import { fib } from '@yors/fib-rs'
console.log(fib(40)) //102334155
```

## Support matrix

### Operating Systems

|                  | node14 | node16 | node18 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      |
| Linux arm gnu    | ✓      | ✓      | ✓      |
| Linux arm64 gnu  | ✓      | ✓      | ✓      |
| Linux arm64 musl | ✓      | ✓      | ✓      |
| Android arm64    | ✓      | ✓      | ✓      |
| Android armv7    | ✓      | ✓      | ✓      |
| FreeBSD x64      | ✓      | ✓      | ✓      |

## workflow for development

```powershell


# code:
# ...
# clone from @napi-rs/package-template
# degit napi-rs/package-template`#main packages/add-rs
degit napi-rs/package-template`#main node-addon-fib-rs
# cd node-addon-fib-rs

# change napi pacakge name:
yarn exec napi rename -n @yors/fib-rs

# install node_module packages with yarn:
yarn

# build:
yarn build

# test:
yarn test
yarn bench


# lint:
yarn lint
yarn format


# github & gh & ghg-ify:
# ...
$repo="ymc-github/node-addon-fib-rs";
$repo_desc="a fib for node in rust with napi-rs/package/template";

$repo_uname=$repo -replace "-","_" -replace "/","_";
$repo_name=$repo  -replace ".*/","";
$repo_user=$repo  -replace "/.*","";

$email=git config user.email;

$repo_user;
$repo_name;

# public
gh repo create $repo_name --public --description "$repo_desc"
# private
# gh repo create $repo_name --private --description "$repo_desc"

# gh repo deploy-key list --repo $repo

# create deploy token
ssh-keygen -C "$email" -f $HOME/.ssh/gh_$repo_uname -t ed25519 -N '""'

# gh - upload github deploy
gh repo deploy-key add $HOME/.ssh/gh_${repo_uname}.pub --repo $repo -w --title deploy;

# set ssh key client (warn: next cmd will overide .ssh/config)
$txt=@"
Host github.com
    User git
    HostName github.com
    PreferredAuthentications publickey
    IdentityFile ~/.ssh/gh_${repo_uname}
"@

set-content -path $HOME/.ssh/config -value $txt
ssh -T git@github.com
# ssh -Tv git@github.com

# set workflow write mode in webui (todo):
# ...

# prepare npm toekn as secret for github workflow:
# ...
# list
# gh secret list --repo $repo

# put
# gh secret set --repo $repo  -f D:\book\secret\npm.token.auto.md
# NPM_TOKEN=xx in npm.token.md


# push:
# ...
# git remote -v
# git remote remove ghg
git remote add ghg git@github.com:$repo.git
git push -u ghg main
git push ghg main
# git push -u ghg main;
git push ghg main;


# publish:
# yarn r/un version
# yarn artticfacts
# yarn prepublishOnly
# ...

# tag & publish & release:
# ...
# git log --oneline

$ver="1.0.0";
$ver="1.0.1";$tagdesc="put package home page"
git tag v$ver HEAD

# git tag -a v$ver -m "version $ver"

# push one tag to remote ghg
git push ghg v$ver

# push all tag to remote ghg
# git push ghg --tags

# git push ghg main

# del tag
git tag -d v$ver
# del remote tag
git push ghg :refs/tags/v$ver


# git HEAD / HEAD^ / HEAD~
# HEAD = HEAD~0 = HEAD^0

# git tag v$ver HEAD
git tag v$ver HEAD^1

git tag -a v$ver -m "version $ver, tag info"


# del
git tag -d v$ver;git push ghg :refs/tags/v$ver

# del release
gh release delete v$ver --repo $repo --yes

# add
# git tag v$ver HEAD -m "version $ver";

# git tag v$ver HEAD -m "version $ver - $tagdesc";

# git add . ;  git commit -m "version $ver";git tag v$ver HEAD -m "version $ver";

git add . ;  git commit -m "$ver";git tag v$ver HEAD -m "$ver";

# $shcode='git log -1 --pretty=%B | grep "^[0-9]\+\.[0-9]\+\.[0-9]\+$"';sh -c "$shcode";
# check if version in cmtlog
$shcode="git log -1 --pretty=%B | grep '^[0-9]\+\.[0-9]\+\.[0-9]\+$'";sh -c "$shcode";


# push to github
git push ghg main ; git push ghg v$ver

# git push ghg main --tags

# list workflow
gh workflow list --repo $repo
# info workflow status
gh workflow view release  --repo $repo
# gh run list --workflow deploy.yml  --repo $repo

# gh workflow view ci  --repo $repo
# gh workflow view lint  --repo $repo

# list
gh release list --repo $repo
# https://cli.github.com/manual/gh_release_list

# del
gh release delete v$ver --repo $repo --yes;
```

## Ability

### Build

After `yarn build/npm run build` command, you can see `package-template.[darwin|win32|linux].node` file in project root. This is the native addon built from [lib.rs](./src/lib.rs).

### Test

With [ava](https://github.com/avajs/ava), run `yarn test/npm run test` to testing native addon. You can also switch to another testing framework if you want.

### CI

With GitHub Actions, each commit and pull request will be built and tested automatically in [`node@14`, `node@16`, `@node18`] x [`macOS`, `Linux`, `Windows`] matrix. You will never be afraid of the native addon broken in these platforms.

### Release

Release native package is very difficult in old days. Native packages may ask developers who use it to install `build toolchain` like `gcc/llvm`, `node-gyp` or something more.

With `GitHub actions`, we can easily prebuild a `binary` for major platforms. And with `N-API`, we should never be afraid of **ABI Compatible**.

The other problem is how to deliver prebuild `binary` to users. Downloading it in `postinstall` script is a common way that most packages do it right now. The problem with this solution is it introduced many other packages to download binary that has not been used by `runtime codes`. The other problem is some users may not easily download the binary from `GitHub/CDN` if they are behind a private network (But in most cases, they have a private NPM mirror).

In this package, we choose a better way to solve this problem. We release different `npm packages` for different platforms. And add it to `optionalDependencies` before releasing the `Major` package to npm.

`NPM` will choose which native package should download from `registry` automatically. You can see [npm](./npm) dir for details. And you can also run `yarn add @napi-rs/package-template` to see how it works.

## Develop requirements

- Install the latest `Rust`
- Install `Node.js@10+` which fully supported `Node-API`
- Install `yarn@1.x`

## Test in local

- yarn
- yarn build
- yarn test

And you will see:

```bash
$ ava --verbose

  ✔ sync function from native code
  ✔ sleep function from native code (201ms)
  ─

  2 tests passed
✨  Done in 1.12s.
```

## Release package

Ensure you have set your **NPM_TOKEN** in the `GitHub` project setting.

In `Settings -> Secrets`, add **NPM_TOKEN** into it.

When you want to release the package:

```
npm version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]

git push
```

GitHub actions will do the rest job for you.

## find related project

- [find @yors/xx in npmjs.com](https://www.npmjs.com/search?q=%40yors)

- [find @yors/xx in yarnpkg.com/search](https://yarnpkg.com/search?q=%40yors)
