import OpenAI from "~/logos/openai-black.svg?react";
import Gemini from "~/logos/gemini.svg?react";

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
];
