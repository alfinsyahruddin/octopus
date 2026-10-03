import type {
  ChoiceItem,
  DecisionModelInfo,
  DecisionModelType,
  EvaluateDecisionRequest,
  EvaluateDecisionResponse,
} from "#lib/types";
import { evaluateDecision, fetchAvailableModels } from "#lib/api";

const STORAGE_KEY = "octopus_canvas_state_v1";

export interface ChoiceTabState {
  instruction: string;
  prompt: string;
  images: string[];
  choices: ChoiceItem[];
}

export interface ScoreTabState {
  instruction: string;
  prompt: string;
  images: string[];
  levels: string[];
}

export interface NoulTabState {
  instruction: string;
  prompt: string;
  images: string[];
  trueDesc: string;
  falseDesc: string;
}

function createDefaultChoiceState(): ChoiceTabState {
  return {
    instruction: "",
    prompt: "",
    images: [],
    choices: [
      { id: "", description: "" },
      { id: "", description: "" },
    ],
  };
}

function createDefaultScoreState(): ScoreTabState {
  return {
    instruction: "",
    prompt: "",
    images: [],
    levels: ["", ""],
  };
}

function createDefaultNoulState(): NoulTabState {
  return {
    instruction: "",
    prompt: "",
    images: [],
    trueDesc: "",
    falseDesc: "",
  };
}

export class CanvasStore {
  activeType = $state<DecisionModelType>("choice");
  selectedModel = $state<string>("clef-flash");
  availableModels = $state<DecisionModelInfo[]>([]);

  choiceState = $state<ChoiceTabState>(createDefaultChoiceState());
  scoreState = $state<ScoreTabState>(createDefaultScoreState());
  noulState = $state<NoulTabState>(createDefaultNoulState());

  choiceResult = $state<EvaluateDecisionResponse | null>(null);
  scoreResult = $state<EvaluateDecisionResponse | null>(null);
  noulResult = $state<EvaluateDecisionResponse | null>(null);

  isEvaluating = $state<boolean>(false);
  isDebouncing = $state<boolean>(false);
  error = $state<string | null>(null);

  get status(): "loading" | "done" | "idle" {
    if (this.isDebouncing || this.isEvaluating) {
      return "loading";
    }
    const currentResult =
      this.activeType === "choice"
        ? this.choiceResult
        : this.activeType === "score"
          ? this.scoreResult
          : this.noulResult;
    if (currentResult) {
      return "done";
    }
    return "idle";
  }

  private debounceTimer: ReturnType<typeof setTimeout> | null = null;
  private abortController: AbortController | null = null;

  constructor() {
    this.loadFromStorage();
  }

  private loadFromStorage() {
    if (typeof window === "undefined") return;
    try {
      const saved = localStorage.getItem(STORAGE_KEY);
      if (saved) {
        const parsed = JSON.parse(saved);
        if (parsed.activeType) this.activeType = parsed.activeType;
        if (parsed.selectedModel) this.selectedModel = parsed.selectedModel;
        if (parsed.choiceState) this.choiceState = parsed.choiceState;
        if (parsed.scoreState) this.scoreState = parsed.scoreState;
        if (parsed.noulState) this.noulState = parsed.noulState;
        if (parsed.choiceResult) this.choiceResult = parsed.choiceResult;
        if (parsed.scoreResult) this.scoreResult = parsed.scoreResult;
        if (parsed.noulResult) this.noulResult = parsed.noulResult;
      }
    } catch (err) {
      console.warn("Failed to load canvas state from localStorage", err);
    }
  }

  saveToStorage() {
    if (typeof window === "undefined") return;
    try {
      const data = {
        activeType: this.activeType,
        selectedModel: this.selectedModel,
        choiceState: this.choiceState,
        scoreState: this.scoreState,
        noulState: this.noulState,
        choiceResult: this.choiceResult,
        scoreResult: this.scoreResult,
        noulResult: this.noulResult,
      };
      localStorage.setItem(STORAGE_KEY, JSON.stringify(data));
    } catch (err) {
      console.warn("Failed to save canvas state to localStorage", err);
    }
  }

  async initModels() {
    try {
      const models = await fetchAvailableModels();
      this.availableModels = models;
      if (
        models.length > 0 &&
        !models.some((m: DecisionModelInfo) => m.id === this.selectedModel)
      ) {
        this.selectedModel = models[0].id;
      }
    } catch (err) {
      console.error("Failed to fetch models from backend", err);
    }
  }

  setActiveType(type: DecisionModelType) {
    this.activeType = type;
    this.error = null;
    this.saveToStorage();
    this.scheduleEvaluate();
  }

  setSelectedModel(model: string) {
    this.selectedModel = model;
    this.saveToStorage();
    this.scheduleEvaluate();
  }

  resetCurrentType() {
    if (this.debounceTimer) {
      clearTimeout(this.debounceTimer);
      this.debounceTimer = null;
    }
    if (this.abortController) {
      this.abortController.abort();
      this.abortController = null;
    }
    this.isDebouncing = false;
    this.isEvaluating = false;

    if (this.activeType === "choice") {
      this.choiceState = createDefaultChoiceState();
      this.choiceResult = null;
    } else if (this.activeType === "score") {
      this.scoreState = createDefaultScoreState();
      this.scoreResult = null;
    } else if (this.activeType === "noul") {
      this.noulState = createDefaultNoulState();
      this.noulResult = null;
    }
    this.error = null;
    this.saveToStorage();
  }

  // Choice Helpers
  addChoice() {
    this.choiceState.choices.push({ id: "", description: "" });
    this.saveToStorage();
    this.scheduleEvaluate();
  }

  removeChoice(index: number) {
    if (this.choiceState.choices.length > 2) {
      this.choiceState.choices.splice(index, 1);
      this.saveToStorage();
      this.scheduleEvaluate();
    }
  }

  updateChoiceId(index: number, id: string) {
    this.choiceState.choices[index].id = id;
    this.saveToStorage();
    this.scheduleEvaluate();
  }

  updateChoiceDesc(index: number, desc: string) {
    this.choiceState.choices[index].description = desc;
    this.saveToStorage();
    this.scheduleEvaluate();
  }

  // Score Helpers
  addScoreLevel() {
    this.scoreState.levels.push("");
    this.saveToStorage();
    this.scheduleEvaluate();
  }

  removeScoreLevel(index: number) {
    if (this.scoreState.levels.length > 2) {
      this.scoreState.levels.splice(index, 1);
      this.saveToStorage();
      this.scheduleEvaluate();
    }
  }

  updateScoreLevel(index: number, levelText: string) {
    this.scoreState.levels[index] = levelText;
    this.saveToStorage();
    this.scheduleEvaluate();
  }

  // Image Helpers
  addImage(base64: string) {
    if (this.activeType === "choice") {
      this.choiceState.images.push(base64);
    } else if (this.activeType === "score") {
      this.scoreState.images.push(base64);
    } else if (this.activeType === "noul") {
      this.noulState.images.push(base64);
    }
    this.saveToStorage();
    this.scheduleEvaluate();
  }

  removeImage(index: number) {
    if (this.activeType === "choice") {
      this.choiceState.images.splice(index, 1);
    } else if (this.activeType === "score") {
      this.scoreState.images.splice(index, 1);
    } else if (this.activeType === "noul") {
      this.noulState.images.splice(index, 1);
    }
    this.saveToStorage();
    this.scheduleEvaluate();
  }

  scheduleEvaluate() {
    this.isDebouncing = true;
    if (this.debounceTimer) {
      clearTimeout(this.debounceTimer);
    }
    this.debounceTimer = setTimeout(() => {
      this.isDebouncing = false;
      this.executeEvaluate();
    }, 1000);
  }

  private async executeEvaluate() {
    if (this.abortController) {
      this.abortController.abort();
    }

    let instruction = "";
    let prompt = "";
    let images: string[] = [];

    if (this.activeType === "choice") {
      instruction = this.choiceState.instruction.trim();
      prompt = this.choiceState.prompt.trim();
      images = this.choiceState.images;

      const validChoices = this.choiceState.choices.filter(
        (c) => c.id.trim().length > 0,
      );
      if (validChoices.length < 2 || !instruction || !prompt) {
        this.choiceResult = null;
        this.saveToStorage();
        return;
      }
    } else if (this.activeType === "score") {
      instruction = this.scoreState.instruction.trim();
      prompt = this.scoreState.prompt.trim();
      images = this.scoreState.images;

      const validLevels = this.scoreState.levels.filter(
        (l) => l.trim().length > 0,
      );
      if (validLevels.length < 2 || !instruction || !prompt) {
        this.scoreResult = null;
        this.saveToStorage();
        return;
      }
    } else if (this.activeType === "noul") {
      instruction = this.noulState.instruction.trim();
      prompt = this.noulState.prompt.trim();
      images = this.noulState.images;

      if (!instruction || !prompt) {
        this.noulResult = null;
        this.saveToStorage();
        return;
      }
    }

    this.abortController = new AbortController();
    this.isEvaluating = true;
    this.error = null;

    try {
      let req: EvaluateDecisionRequest;

      if (this.activeType === "choice") {
        const validChoices = this.choiceState.choices.filter(
          (c) => c.id.trim().length > 0,
        );
        req = {
          model: this.selectedModel,
          model_type: "choice",
          instruction,
          prompt,
          images,
          choices: validChoices.map((c) => ({
            id: c.id.trim(),
            description: c.description?.trim() || undefined,
          })),
        };
      } else if (this.activeType === "score") {
        const validLevels = this.scoreState.levels.filter(
          (l) => l.trim().length > 0,
        );
        req = {
          model: this.selectedModel,
          model_type: "score",
          instruction,
          prompt,
          images,
          score_levels: validLevels.map((l) => l.trim()),
        };
      } else {
        req = {
          model: this.selectedModel,
          model_type: "noul",
          instruction,
          prompt,
          images,
          noul_criteria: {
            true_desc: this.noulState.trueDesc.trim() || undefined,
            false_desc: this.noulState.falseDesc.trim() || undefined,
          },
        };
      }

      const res = await evaluateDecision(req, this.abortController.signal);

      if (this.activeType === "choice") {
        this.choiceResult = res;
      } else if (this.activeType === "score") {
        this.scoreResult = res;
      } else if (this.activeType === "noul") {
        this.noulResult = res;
      }
      this.saveToStorage();
    } catch (err: unknown) {
      if (err instanceof DOMException && err.name === "AbortError") {
        return;
      }
      console.error("Evaluation failed:", err);
      this.error = err instanceof Error ? err.message : "Evaluation failed";
    } finally {
      this.isEvaluating = false;
    }
  }
}

export const canvasStore = new CanvasStore();
