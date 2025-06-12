import OpenAI from "~/logos/openai-black.svg?react";

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
    ],
  },
];
