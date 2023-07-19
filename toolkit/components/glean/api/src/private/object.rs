// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{CommonMetricData, MetricId};

use crate::ipc::need_ipc;

use glean::traits::ObjectObject;

/// An object metric.
pub enum ObjectMetric<K> {
    Parent {
        id: MetricId,
        inner: glean::private::ObjectMetric<K>,
    },
    Child(MetricId),
}

impl<K: ObjectObject> ObjectMetric<K> {
    /// Create a new object metric.
    pub fn new(id: MetricId, meta: CommonMetricData) -> Self {
        if need_ipc() {
            ObjectMetric::Child(id)
        } else {
            let inner = glean::private::ObjectMetric::new(meta);
            ObjectMetric::Parent { id, inner }
        }
    }

    pub fn set(&self, value: K) {
        match self {
            ObjectMetric::Parent { inner, .. } => {
                inner.set(value);
            }
            ObjectMetric::Child(_) => {
                log::error!("Unable to set object metric in non-main process. Ignoring.");
                // TODO: Record an error.
            }
        };
    }

    pub fn set_str(&self, value: String) {
        match self {
            ObjectMetric::Parent { inner, .. } => {
                log::info!("Setting object metric to: {}", value);
                inner.set_str(value);
            }
            ObjectMetric::Child(_) => {
                log::error!("Unable to set object metric in non-main process. Ignoring.");
                // TODO: Record an error.
            }
        };
    }

    pub fn test_get_value<'a, S: Into<Option<&'a str>>>(
        &self,
        ping_name: S,
    ) -> Option<String> {
        match self {
            ObjectMetric::Parent { inner, .. } => inner.test_get_value(ping_name),
            ObjectMetric::Child(_) => {
                panic!("Cannot get test value for object metric in non-parent process!",)
            }
        }
    }

    pub fn test_get_num_recorded_errors(&self, error: glean::ErrorType) -> i32 {
        match self {
            ObjectMetric::Parent { inner, .. } => inner.test_get_num_recorded_errors(error),
            ObjectMetric::Child(c) => panic!(
                "Cannot get the number of recorded errors for {:?} in non-parent process!",
                c.0
            ),
        }
    }
}
