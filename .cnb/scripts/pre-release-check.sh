#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=.cnb/scripts/release-common.sh
source "${SCRIPT_DIR}/release-common.sh"

require_cmd git

git fetch --tags --force origin

bash "${SCRIPT_DIR}/check-release-tag.sh"
bash "${SCRIPT_DIR}/ci-check.sh" test
