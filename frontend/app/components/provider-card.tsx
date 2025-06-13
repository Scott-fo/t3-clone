import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "~/components/ui/card";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "~/components/ui/alert-dialog";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { Loader2, Trash } from "lucide-react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { type ApiKey, type CreateApiKeyPayload } from "~/domain/api-key";
import { useCreateApiKey, useDeleteApiKey } from "~/hooks/use-api-keys";

type Props = {
  provider: "openai" | "google";
  placeholder: string;
  existingKey?: ApiKey;
};

const schema = z.object({
  key: z.string().min(1, "Key is required"),
});

type FormValues = z.infer<typeof schema>;

export default function ProviderCard({
  provider,
  existingKey,
  placeholder,
}: Props) {
  const create = useCreateApiKey();
  const del = useDeleteApiKey();

  const form = useForm<FormValues>({
    resolver: zodResolver(schema),
    defaultValues: { key: "" },
  });

  const onSubmit = (values: FormValues) => {
    const payload: CreateApiKeyPayload = { provider, key: values.key };
    create.mutate(payload, { onSuccess: () => form.reset() });
  };

  return (
    <Card className="w-full max-w-3xl">
      <CardHeader>
        <CardTitle className="flex items-center justify-between">
          {provider.toUpperCase()}
          {existingKey ? (
            <Badge variant="default">Connected</Badge>
          ) : (
            <Badge variant="secondary">Not connected</Badge>
          )}
        </CardTitle>
        <CardDescription>
          Provide your {provider.toUpperCase()} API key so the server can make
          calls on your behalf.
        </CardDescription>
      </CardHeader>

      <CardContent>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
          <Input
            {...form.register("key")}
            placeholder={placeholder}
            type="password"
            disabled={create.isPending}
          />
          {form.formState.errors.key && (
            <p className="text-sm text-red-600">
              {form.formState.errors.key.message}
            </p>
          )}
          <div className="flex w-full justify-end gap-x-2">
            {existingKey && (
              <AlertDialog>
                <AlertDialogTrigger asChild>
                  <Button variant="destructive" disabled={del.isPending}>
                    {del.isPending ? (
                      <Loader2 className="mr-1 h-4 w-4 animate-spin" />
                    ) : (
                      <Trash className="mr-1 h-4 w-4" />
                    )}
                    Delete
                  </Button>
                </AlertDialogTrigger>
                <AlertDialogContent>
                  <AlertDialogHeader>
                    <AlertDialogTitle>
                      Are you absolutely sure?
                    </AlertDialogTitle>
                    <AlertDialogDescription>
                      This action cannot be undone. This will permanently delete
                      your stored API key for{" "}
                      {existingKey.provider.toUpperCase()}.
                    </AlertDialogDescription>
                  </AlertDialogHeader>
                  <AlertDialogFooter>
                    <AlertDialogCancel>Cancel</AlertDialogCancel>
                    <AlertDialogAction
                      onClick={() => del.mutate(existingKey.id)}
                    >
                      Continue
                    </AlertDialogAction>
                  </AlertDialogFooter>
                </AlertDialogContent>
              </AlertDialog>
            )}
            <Button type="submit" disabled={create.isPending} className="">
              {create.isPending && (
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
              )}
              {existingKey ? "Update" : "Save"}
            </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  );
}
