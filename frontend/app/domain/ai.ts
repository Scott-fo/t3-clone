import OpenAI from "~/logos/openai-black.svg?react";
import Gemini from "~/logos/gemini.svg?react";
import Anthropic from "~/logos/anthropic.svg?react";
import OpenRouter from "~/logos/openrouter.svg?react";

export type Reasoning = "high" | "medium" | "low";

type SupportedModel = {
  model: string;
  display: string;
  reasoning?: boolean;
};

type SupportedModels = {
  provider: string;
  logo: any;
  models: SupportedModel[];
};

export const supportedModels: SupportedModels[] = [
  {
    provider: "openai",
    logo: OpenAI,
    models: [
      {
        model: "o3",
        display: "o3",
        reasoning: true,
      },
      {
        model: "o4-mini",
        display: "o4 mini",
        reasoning: true,
      },
      {
        model: "o3-mini",
        display: "o3 mini",
        reasoning: true,
      },
      {
        model: "gpt-4.1",
        display: "GPT 4.1",
      },
      {
        model: "gpt-4.1-mini",
        display: "GPT 4.1 mini",
      },
      {
        model: "gpt-4.1-nano",
        display: "GPT 4.1 nano",
      },
      {
        model: "gpt-4o",
        display: "GPT 4o",
      },
    ],
  },
  {
    provider: "google",
    logo: Gemini,
    models: [
      {
        model: "gemini-2.5-pro-preview-06-05",
        display: "Gemini 2.5 Pro",
        reasoning: false,
      },
      {
        model: "gemini-2.5-flash-preview-05-20",
        display: "Gemini 2.5 Flash",
        reasoning: false,
      },
      {
        model: "gemini-2.0-flash",
        display: "Gemini 2.0 Flash",
        reasoning: false,
      },
    ],
  },
  {
    provider: "anthropic",
    logo: Anthropic,
    models: [
      {
        model: "claude-3-5-haiku-latest",
        display: "Claude 3.5 Haiku",
        reasoning: false,
      },
      {
        model: "claude-sonnet-4-20250514",
        display: "Claude Sonnet 4",
        reasoning: false,
      },
      {
        model: "claude-opus-4-20250514",
        display: "Claude Opus 4",
        reasoning: false,
      },
    ],
  },
  {
    provider: "openrouter",
    logo: OpenRouter,
    models: [
      {
        model: "deepseek/deepseek-r1-0528-qwen3-8b:free",
        display: "DeepSeek R1 8b (OpenRouter)",
        reasoning: false,
      },
      {
        model: "google/gemini-2.0-flash-exp:free",
        display: "Gemini 2.0 Flash (OpenRouter)",
        reasoning: false,
      },
    ],
  },
];
