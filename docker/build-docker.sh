#!/bin/bash
  set -o errexit   # abort on nonzero exitstatus
  set -o nounset   # abort on unbound variable
  set -o pipefail  # don't hide errors within pipes

if [ -z "${1}" ]
then
  echo "No Author/Repo name provided!"
  return 1
fi

if [ -z "${2}" ]
then
  echo "No Image tagName provided!"
  return 1
fi

author="${1}"
tagName="${2}"

commitSha="${3:-$(git rev-parse HEAD)}"
imageCreationDateTime=$(date +"%Y-%m-%dT%H:00:00%z")

script_path="$(dirname "$(realpath "$0")")"

bot_dockerfile_dir="${script_path}"/../panel/
panel_dockerfile_dir="${script_path}"/../bot/

docker image build --tag "${author}"/talium_panel:"${tagName}" --build-arg COMMIT_SHA="${commitSha}" --build-arg TAG_NAME="${tagName}" --build-arg CREATED_AT="${imageCreationDateTime}" "${bot_dockerfile_dir}"

docker image build --tag "${author}"/talium_bot:"${tagName}" --build-arg COMMIT_SHA="${commitSha}" --build-arg TAG_NAME="${tagName}" --build-arg CREATED_AT="${imageCreationDateTime}" "${panel_dockerfile_dir}"
