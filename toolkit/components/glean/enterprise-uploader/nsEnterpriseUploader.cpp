/* -*- Mode: C++; tab-width: 8; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* vim: set ts=8 sts=2 et sw=2 tw=80: */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "mozilla/ClearOnShutdown.h"
#include "mozilla/StaticPtr.h"
#include "nsCOMPtr.h"
#include "nsEnterpriseUploader.h"

// This anonymous namespace prevents outside C++ code from improperly accessing
// these implementation details.
namespace {
extern "C" {
// Implemented in Rust.
void new_enterprise_uploader_service(nsIEnterpriseUploader** result);
}

static mozilla::StaticRefPtr<nsIEnterpriseUploader> sService;
}  // namespace


namespace mozilla::glean {

already_AddRefed<nsIEnterpriseUploader> GetEnterpriseUploaderService() {
  nsCOMPtr<nsIEnterpriseUploader> service;

  if (sService) {
    service = sService;
  } else {
    new_enterprise_uploader_service(getter_AddRefs(service));
    sService = service;
    mozilla::ClearOnShutdown(&sService);
  }

  return service.forget();
}

}  // namespace mozilla::glean
