#!/bin/sh
# Vercel's Ignored Build Step for the website. Exit 1 means build, exit 0 means
# skip: https://vercel.com/docs/project-configuration/vercel-json#ignorecommand
#
# Compares HEAD with the last successful deployment of this branch
# (VERCEL_GIT_PREVIOUS_SHA, empty on a branch's first deployment), or with
# HEAD^ when there is none. Builds whenever it cannot tell, for example when
# the shallow clone does not contain the commit to compare with.

set -u

# The paths the site reads. ":/" makes each one relative to the repository
# root, so the script works from website/, which is where Vercel runs it.
set -- \
  ":/website" \
  ":/design/tokens" \
  ":/assets/brand" \
  ":/spec" \
  ":/CHANGELOG.md" \
  ":/conformance/vectors/cited" \
  ":/bindings/js/package.json"

base="${VERCEL_GIT_PREVIOUS_SHA:-}"
if [ -z "$base" ]; then
  base="HEAD^"
fi

if ! git rev-parse --quiet --verify "$base^{commit}" >/dev/null; then
  echo "should-build: cannot find $base in this clone, so building"
  exit 1
fi

git diff --quiet "$base" HEAD -- "$@"
status=$?
if [ "$status" -eq 0 ]; then
  echo "should-build: nothing the site reads changed since $base, so skipping"
  exit 0
fi
if [ "$status" -eq 1 ]; then
  echo "should-build: the site's inputs changed since $base, so building"
  exit 1
fi
echo "should-build: git diff failed with status $status, so building"
exit 1
