# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

import sys
from os import path

import mozunit

# Shenanigans to import the metrics index's list of metrics.yamls
FOG_ROOT_PATH = path.abspath(
    path.join(path.dirname(__file__), path.pardir, path.pardir)
)
sys.path.append(FOG_ROOT_PATH)
from metrics_index import firefox_desktop_metrics, gecko_metrics


def test_no_metrics_file_duplicated():
    """
    When metrics files are listed in both `firefox_desktop_metrics` and `gecko_metrics` they are duplicated
    and that will lead to problems in probe-scraper.

    They should only be listed in one of them.
    """

    ffx_metrics = set(firefox_desktop_metrics)
    gko_metrics = set(gecko_metrics)

    in_both = ffx_metrics & gko_metrics

    assert not in_both, (
        "Files duplicated in both firefox_desktop_metrics and gecko_metrics are not allowed"
    )


if __name__ == "__main__":
    mozunit.main()
