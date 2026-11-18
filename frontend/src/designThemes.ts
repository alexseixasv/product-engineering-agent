export type DesignThemeId = "pulseboard" | "deal-feed";

export type DesignThemeOption = {
  id: DesignThemeId;
  name: string;
  tagline: string;
  accents: [string, string, string];
  specPath: string;
};

export const DESIGN_THEMES: DesignThemeOption[] = [
  {
    id: "pulseboard",
    name: "Pulseboard studio",
    tagline:
      "Warm paper canvas, serif display, terracotta accent — aligned with the sample-app fixture.",
    accents: ["#c44900", "#14213d", "#f8f5f0"],
    specPath: "fixtures/sample-app (design-system primitives)",
  },
  {
    id: "deal-feed",
    name: "Deal Feed Density",
    tagline:
      "mydealz-inspired cool-gray feed: brand green actions/prices, hot/cold temperature pills, full-round chrome.",
    accents: ["#30a83c", "#e53935", "#1e88e5"],
    specPath: "docs/design/mydealz-design-system.md",
  },
];

export function parseThemeId(raw: string | null): DesignThemeId | null {
  if (raw === "pulseboard" || raw === "deal-feed") return raw;
  return null;
}
