import type { DesignThemeId, DesignThemeOption } from "./designThemes";
import { DESIGN_THEMES } from "./designThemes";

type Props = {
  selected: DesignThemeId | null;
  onSelect: (id: DesignThemeId) => void;
};

export function DesignThemeChooser({ selected, onSelect }: Props) {
  return (
    <div className="mx-auto max-w-5xl px-4 py-10" data-testid="design-theme-chooser">
      <header className="mb-8">
        <p className="text-sm font-medium uppercase tracking-wide text-ink/50">
          Visual proposal
        </p>
        <h1 className="mt-1 font-display text-3xl font-bold text-ink">
          Choose a design direction
        </h1>
        <p className="mt-2 max-w-2xl text-sm text-ink/70">
          Pick how the analytics feature proposal should look for product review.
          Both options keep the same information architecture; only the visual
          system changes.
        </p>
      </header>

      <div className="grid gap-4 md:grid-cols-2">
        {DESIGN_THEMES.map((theme) => (
          <ThemeCard
            key={theme.id}
            theme={theme}
            active={selected === theme.id}
            onSelect={() => onSelect(theme.id)}
          />
        ))}
      </div>
    </div>
  );
}

function ThemeCard({
  theme,
  active,
  onSelect,
}: {
  theme: DesignThemeOption;
  active: boolean;
  onSelect: () => void;
}) {
  return (
    <button
      type="button"
      onClick={onSelect}
      data-testid={`theme-option-${theme.id}`}
      className={`rounded-lg border p-5 text-left transition ${
        active
          ? "border-ink bg-white shadow-sm ring-2 ring-ink"
          : "border-line bg-white/70 hover:border-ink/40 hover:bg-white"
      }`}
    >
      <div className="mb-4 flex gap-2">
        {theme.accents.map((color) => (
          <span
            key={color}
            className="h-8 w-8 rounded-full border border-black/10"
            style={{ backgroundColor: color }}
            aria-hidden
          />
        ))}
      </div>
      <h2 className="font-display text-xl font-semibold text-ink">{theme.name}</h2>
      <p className="mt-2 text-sm leading-relaxed text-ink/70">{theme.tagline}</p>
      <p className="mt-3 text-xs text-ink/50">Spec: {theme.specPath}</p>
      <span className="mt-4 inline-flex text-sm font-medium text-accent">
        {active ? "Selected — scroll to preview" : "Use this design →"}
      </span>
    </button>
  );
}
