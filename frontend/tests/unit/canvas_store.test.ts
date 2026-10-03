import { describe, it, expect, vi, beforeEach } from "vitest";
import { CanvasStore } from "#lib/state/canvas.svelte";

describe("CanvasStore Status & Persistence", () => {
  beforeEach(() => {
    localStorage.clear();
    vi.restoreAllMocks();
  });

  it("initializes with IDLE status when no results exist", () => {
    const store = new CanvasStore();
    expect(store.status).toBe("idle");
  });

  it("sets status to loading when debouncing", () => {
    vi.useFakeTimers();
    const store = new CanvasStore();

    store.scheduleEvaluate();
    expect(store.isDebouncing).toBe(true);
    expect(store.status).toBe("loading");

    vi.runAllTimers();
    vi.useRealTimers();
  });

  it("saves and restores results to/from localStorage across reloads", () => {
    const store1 = new CanvasStore();
    store1.activeType = "choice";
    store1.choiceState.instruction = "Test instruction";
    store1.choiceState.prompt = "Test prompt";
    store1.choiceResult = {
      model: "clef-flash",
      model_type: "choice",
      result: {
        type: "choice",
        choice: "A",
        probabilities: { A: 0.9, B: 0.1 },
        confidence: 0.85,
      },
      ai_duration_ms: 25.0,
    };
    store1.saveToStorage();

    // Instantiate a new store simulating page reload
    const store2 = new CanvasStore();
    expect(store2.activeType).toBe("choice");
    expect(store2.choiceState.instruction).toBe("Test instruction");
    expect(store2.choiceState.prompt).toBe("Test prompt");
    expect(store2.choiceResult).toEqual(store1.choiceResult);
    expect(store2.status).toBe("done");
  });

  it("resets state and clears results back to IDLE status", () => {
    const store = new CanvasStore();
    store.choiceResult = {
      model: "clef-flash",
      model_type: "choice",
      result: {
        type: "choice",
        choice: "A",
        probabilities: { A: 0.9, B: 0.1 },
        confidence: 0.9,
      },
      ai_duration_ms: 10.0,
    };
    expect(store.status).toBe("done");

    store.resetCurrentType();
    expect(store.choiceResult).toBeNull();
    expect(store.status).toBe("idle");
  });
});
