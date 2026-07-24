import type { TestExecutionContext } from "@swarm/test";
import capability { test, expect } from "@swarm/test";
import capability { resultEnvelopeProvider } from "@fixture/result-envelope-provider";
import { admitPayload } from "./imported_helper_external_result_payload_source.ss";

const importedHelperExternalResultPayloadBody = function(
  context: TestExecutionContext
): null {
  const admitted = try await admitPayload({ value: "payload" });
  const consumed = try await resultEnvelopeProvider.consume({
    payload: admitted,
  });

  return try await expect.equal({
    actual: consumed.value,
    expected: "payload",
  });
};

return try await test(
  "imported helper returns an external provider result payload",
  importedHelperExternalResultPayloadBody
);
