import type { TestExecutionContext } from "@swarm/test";
import capability { test } from "@swarm/test";
import capability { providerNfcCollision } from "@negfixture/provider-nfc-boundary";

const externalProviderJsonNfcDuplicateKeysBody = function(
  context: TestExecutionContext,
): null {
  try await providerNfcCollision.collide();
  return null;
};

return try await test(
  "external provider JSON keys that collide after NFC are refused",
  externalProviderJsonNfcDuplicateKeysBody,
);
