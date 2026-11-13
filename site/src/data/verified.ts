// Typed view of verified.json, which studio-verify (../../verify) writes from
// real rustc runs. Nothing in the site states a compiler result on its own.
import data from "./verified.json";

export interface VerifiedExample {
  name: string;
  /** "pass", or the single error code rustc must report (e.g. "E0382"). */
  expect: string;
  source: string;
  stdout: string;
  stderr: string;
}

export const verified = data as {
  toolchain: string;
  rustc: string;
  edition: string;
  starter: { command: string; stdout: string };
  examples: VerifiedExample[];
};

export function example(name: string): VerifiedExample {
  const found = verified.examples.find((e) => e.name === name);
  // Fail the build, not the page: a missing example is an authoring error.
  if (!found) throw new Error(`No verified example named "${name}"`);
  return found;
}

export const passes = (e: VerifiedExample) => e.expect === "pass";
