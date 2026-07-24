import type { TestExecutionContext } from "@swarm/test";
import capability { expect, test } from "@swarm/test";
import capability { resultEnvelopeProvider } from "@fixture/result-envelope-provider";

const externalProviderJsonTextNfcBody = function(
  context: TestExecutionContext,
): null {
  const admitted = try await resultEnvelopeProvider.normalizeTextAtProviderBoundary();

  return try await expect.equal({
    actual: admitted.value,
    expected: "café",
  });
};

return try await test(
  "external provider JSON text is NFC at ingress",
  externalProviderJsonTextNfcBody,
);
