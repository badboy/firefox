/* -*- Mode: C++; tab-width: 8; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* vim: set ts=8 sts=2 et sw=2 tw=80: */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "mozilla/glean/bindings/Object.h"

#include "Common.h"
#include "mozilla/Components.h"
#include "mozilla/dom/ToJSValue.h"
#include "mozilla/Logging.h"
#include "nsIClassInfoImpl.h"
#include "js/JSON.h"
#include "jsapi.h"

namespace mozilla::glean {

NS_IMPL_CLASSINFO(GleanObject, nullptr, 0, {0})
NS_IMPL_ISUPPORTS_CI(GleanObject, nsIGleanObject)

static bool JSONCreator(const char16_t* aBuf, uint32_t aLen, void* aData) {
  nsAString* result = static_cast<nsAString*>(aData);
  result->Append(aBuf, aLen);
  return true;
}

NS_IMETHODIMP
GleanObject::Set(JS::HandleValue aObj, JSContext* aCx) {
  if (!aObj.isObject()) {
    return NS_ERROR_INVALID_ARG;
  }

  JS::Rooted<JS::Value> value(aCx, aObj);
  nsAutoString serializedValue;
  bool res = JS_Stringify(aCx, &value, nullptr, JS::NullHandleValue,
                          JSONCreator, &serializedValue);
  if (!res) {
    return NS_ERROR_INVALID_ARG;
  }

  NS_ConvertUTF16toUTF8 payload(serializedValue);
  printf("jer. Got object (stringified): |%s|\n", payload.get());
  mObject.SetStr(payload);

  return NS_OK;
}

NS_IMETHODIMP
GleanObject::TestGetValue(const nsACString& aStorageName, JSContext* aCx,
                          JS::MutableHandle<JS::Value> aResult) {
  auto result = mObject.TestGetValue(aStorageName);
  if (result.isErr()) {
    aResult.set(JS::UndefinedValue());
    LogToBrowserConsole(nsIScriptError::errorFlag,
                        NS_ConvertUTF8toUTF16(result.unwrapErr()));
    return NS_ERROR_LOSS_OF_SIGNIFICANT_DATA;
  }
  auto optresult = result.unwrap();
  if (optresult.isNothing()) {
    aResult.set(JS::UndefinedValue());
  } else {
    const NS_ConvertUTF8toUTF16 str(optresult.ref());
    aResult.set(
        JS::StringValue(JS_NewUCStringCopyN(aCx, str.Data(), str.Length())));
  }
  return NS_OK;
}

}  // namespace mozilla::glean
