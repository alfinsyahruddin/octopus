export type DecisionModelType = "choice" | "score" | "noul";

export interface DecisionModelInfo {
  id: string;
  name: string;
  description: string;
  supported_types: DecisionModelType[];
  supports_vision: boolean;
}

export interface ChoiceItem {
  id: string;
  description?: string;
}

export interface NoulCriteria {
  true_desc?: string;
  false_desc?: string;
}

export interface EvaluateDecisionRequest {
  model: string;
  model_type: DecisionModelType;
  instruction: string;
  prompt: string;
  images: string[];
  choices?: ChoiceItem[];
  score_levels?: string[];
  noul_criteria?: NoulCriteria;
}

export interface ChoiceResult {
  choice: string;
  probabilities: Record<string, number>;
  confidence: number;
}

export interface ScoreResult {
  score: number;
  legend: Record<string, string>;
  probabilities: Record<string, number>;
  confidence: number;
}

export interface NoulResult {
  noul: number;
}

export type DecisionResultUnion =
  | ({ type: "choice" } & ChoiceResult)
  | ({ type: "score" } & ScoreResult)
  | ({ type: "noul" } & NoulResult);

export interface EvaluateDecisionResponse {
  model: string;
  model_type: DecisionModelType;
  result: DecisionResultUnion;
  ai_duration_ms: number;
}
