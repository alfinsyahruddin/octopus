import type {
  DecisionModelInfo,
  EvaluateDecisionRequest,
  EvaluateDecisionResponse,
} from "#lib/types";

const BASE_URL = import.meta.env.PUBLIC_API_BASE_URL || "http://localhost:8000";

export class ApiError extends Error {
  constructor(
    public override message: string,
    public status: number,
  ) {
    super(message);
    this.name = "ApiError";
  }
}

interface ApiEnvelope<T> {
  data: T | null;
  status: number;
  message: string | null;
  timestamp: string;
}

export async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    Accept: "application/json",
    ...(init?.headers as Record<string, string> | undefined),
  };

  const res = await fetch(`${BASE_URL}${path}`, {
    ...init,
    headers,
  });

  let envelope: ApiEnvelope<T>;
  try {
    envelope = await res.json();
  } catch {
    throw new ApiError(
      "An unexpected response format was received from the server",
      res.status,
    );
  }

  if (!res.ok || envelope.status >= 400 || envelope.data === null) {
    throw new ApiError(
      envelope.message || "An unexpected error occurred",
      envelope.status || res.status,
    );
  }

  return envelope.data;
}

export async function fetchAvailableModels(): Promise<DecisionModelInfo[]> {
  return request<DecisionModelInfo[]>("/canvas/models");
}

export async function evaluateDecision(
  payload: EvaluateDecisionRequest,
  signal?: AbortSignal,
): Promise<EvaluateDecisionResponse> {
  return request<EvaluateDecisionResponse>("/canvas/evaluate", {
    method: "POST",
    body: JSON.stringify(payload),
    signal,
  });
}
