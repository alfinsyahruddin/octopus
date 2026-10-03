import { describe, it, expect, vi, beforeEach } from "vitest";
import { fetchAvailableModels, evaluateDecision, ApiError } from "#lib/api";

describe("API Client", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("successfully fetches available models", async () => {
    const mockModels = [
      {
        id: "clef-flash",
        name: "Clef Flash 9B",
        description: "Cloudflare model",
        supported_types: ["choice", "score", "noul"],
        supports_vision: true,
      },
    ];

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        data: mockModels,
        status: 200,
        message: null,
        timestamp: new Date().toISOString(),
      }),
    } as unknown as Response);

    const result = await fetchAvailableModels();
    expect(result).toEqual(mockModels);
    expect(global.fetch).toHaveBeenCalledWith(
      "http://localhost:8000/canvas/models",
      expect.anything(),
    );
  });

  it("evaluates decision and returns unwrapped envelope data", async () => {
    const mockResponse = {
      model: "clef-flash",
      model_type: "noul",
      result: { type: "noul", noul: 0.95 },
      ai_duration_ms: 35.2,
    };

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        data: mockResponse,
        status: 200,
        message: null,
        timestamp: new Date().toISOString(),
      }),
    } as unknown as Response);

    const result = await evaluateDecision({
      model: "clef-flash",
      model_type: "noul",
      instruction: "Is this urgent?",
      prompt: "Help immediately",
      images: [],
    });

    expect(result).toEqual(mockResponse);
  });

  it("throws ApiError on server error response", async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 400,
      json: async () => ({
        data: null,
        status: 400,
        message: "Prompt cannot be empty",
        timestamp: new Date().toISOString(),
      }),
    } as unknown as Response);

    await expect(
      evaluateDecision({
        model: "clef-flash",
        model_type: "choice",
        instruction: "Instruction",
        prompt: "",
        images: [],
      }),
    ).rejects.toThrow(ApiError);
  });
});
