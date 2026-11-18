import { useMemo, useState } from "react";
import { AnalyticsPrototypeDealFeed } from "./AnalyticsPrototypeDealFeed";
import { AnalyticsPrototypePulseboard } from "./AnalyticsPrototypePulseboard";
import { DesignThemeChooser } from "./DesignThemeChooser";
import { parseThemeId, type DesignThemeId } from "./designThemes";

function readThemeFromUrl(): DesignThemeId | null {
  const params = new URLSearchParams(window.location.search);
  return parseThemeId(params.get("theme"));
}

export function AnalyticsPrototype() {
  const initial = useMemo(() => readThemeFromUrl(), []);
  const [theme, setTheme] = useState<DesignThemeId | null>(initial);

  function selectTheme(id: DesignThemeId) {
    setTheme(id);
    const url = new URL(window.location.href);
    url.searchParams.set("theme", id);
    window.history.replaceState({}, "", url);
  }

  return (
    <div data-testid="analytics-prototype">
      <DesignThemeChooser selected={theme} onSelect={selectTheme} />
      {theme === "pulseboard" && <AnalyticsPrototypePulseboard />}
      {theme === "deal-feed" && <AnalyticsPrototypeDealFeed />}
      {theme && (
        <div className="mx-auto max-w-5xl px-4 pb-10">
          <button
            type="button"
            className="text-sm font-medium text-ink/60 underline-offset-2 hover:underline"
            onClick={() => {
              setTheme(null);
              const url = new URL(window.location.href);
              url.searchParams.delete("theme");
              window.history.replaceState({}, "", url);
            }}
          >
            ← Change design direction
          </button>
        </div>
      )}
    </div>
  );
}
