import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "~/components/ui/dialog";
import { Input } from "~/components/ui/input";

import { useServiceDialog } from "./use-service-dialog";

import { Button } from "~/components/ui/button";
import { Badge } from "~/components/ui/badge";
import type { Service } from "~/types/service";
import type { Rule } from "~/types/rule";

import { toast } from "~/components/ui/toast/use-toast";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "~/components/ui/form";
import { Checkbox } from "~/components/ui/checkbox";
import axios from "axios";
import type { RuleWithChecked } from "./types";

const getForm = (input?: Service | null, allRules?: Rule[]) => {
  return {
    id: input?.id || 0,
    name: input?.name || "",
    port: input?.port || 0,
    rules:
      allRules?.map((ar) => ({
        ...ar,
        checked: !!input?.rules?.find(
          (r) =>
            r.id === ar.id &&
            r.color === ar.color &&
            r.regexp === ar.regexp &&
            r.name === ar.name
        ),
      })) || ([] as RuleWithChecked[]),
    rulesToCreate: [] as Rule[],
    rulesToDelete: [] as Rule[],
  };
};

export const ServiceDialog = defineComponent({
  name: "ServiceDialog",
  props: {
    data: {
      type: Object as () => Service,
      required: false,
    },
    ruleList: {
      type: Array<Rule>,
      required: false,
    },
    whenSubmitted: {
      type: Function,
      required: true,
    },
    whenClose: {
      type: Function,
      required: true,
    },
  },
  setup(props) {
    const serviceSidebar = useServiceDialog();

    const isSubmitting = ref(false);
    const isOpen = ref(true);

    const isNew = computed(() => {
      return !props.data?.id;
    });

    const form = ref(getForm(props.data, props.ruleList));

    const isFormDataValid = computed(() => {
      return !!form.value.name.length && !!form.value.port;
    });

    const disabledSubmit = computed(() => {
      return !isFormDataValid.value || isSubmitting.value;
    });

    const actionText = computed(() => {
      return isNew.value ? "Create" : "Update";
    });

    const getServiceToUpsert = () => {
      return {
        service: {
          id: form.value?.id,
          name: form.value?.name,
          port: form.value?.port,
        },
        rule_ids: form.value?.rulesToCreate.map((rule) => rule.id),
      };
    };

    const getRulesToDelete = () => {
      return {
        service_id: form.value?.id,
        rule_ids: form.value.rulesToDelete.map((rule) => rule.id),
      };
    };

    const handleSubmit = async () => {
      if (!isFormDataValid.value) return;

      try {
        isSubmitting.value = true;

        await serviceSidebar.upsertService(getServiceToUpsert());

        if (getRulesToDelete().rule_ids.length) {
          await serviceSidebar.deleteServiceToRules(getRulesToDelete());
        }

        props.whenSubmitted?.();

        clearForm();

        toast({
          title: "Success",
        });
      } catch (error) {
        if (axios.isAxiosError(error)) {
          toast({
            title: "Error",
            variant: "destructive",
            description: error.response?.data,
          });
        }
      } finally {
        whenClose();
      }
    };

    const handleDelete = async () => {
      try {
        isSubmitting.value = true;

        const serviceID = form.value?.id;

        await serviceSidebar.deleteService(serviceID);

        // FIX: fix form with rules
        const ruleIDsToDelete = props.data?.rules?.map((r) => r.id);
        if (!!ruleIDsToDelete && ruleIDsToDelete.length > 0) {
          await serviceSidebar.deleteServiceToRules({
            service_id: serviceID,
            rule_ids: ruleIDsToDelete,
          });
        }

        props.whenSubmitted?.();

        clearForm();

        toast({
          title: "Success",
        });
      } catch (error) {
        if (axios.isAxiosError(error)) {
          toast({
            title: "Error",
            variant: "destructive",
            description: error.response?.data,
          });
        }
      } finally {
        whenClose();
      }
    };

    const clearForm = () => {
      form.value = getForm();
    };

    const whenClose = () => {
      isSubmitting.value = false;
      isOpen.value = false;

      props.whenClose();
    };

    return () => (
      <Dialog open={isOpen.value}>
        <DialogContent
          onInteractOutside={() => {
            whenClose();
          }}
        >
          <DialogHeader>
            <DialogTitle>{actionText.value} service</DialogTitle>
            <DialogDescription></DialogDescription>
          </DialogHeader>
          <Form validationSchema={form}>
            <form class="w-2/3 space-y-6">
              <FormField name="name">
                <FormItem>
                  <FormLabel>Name</FormLabel>
                  <FormControl>
                    <Input
                      type="text"
                      placeholder="service name"
                      onChange={(event: any) => {
                        form.value.name = String(event.target.value);
                      }}
                      modelValue={form.value.name}
                    />
                  </FormControl>
                  <FormDescription></FormDescription>
                  <FormMessage />
                </FormItem>
              </FormField>
              <FormField name="port">
                <FormItem>
                  <FormLabel>Port</FormLabel>
                  <FormControl>
                    <Input
                      type="number"
                      placeholder="service port"
                      onChange={(event: any) => {
                        form.value.port = Number(event.target.value);
                      }}
                      modelValue={form.value.port > 0 ? form.value.port : ""}
                    />
                  </FormControl>
                  <FormDescription></FormDescription>
                  <FormMessage />
                </FormItem>
              </FormField>
              <FormField name="rules">
                <FormItem>
                  <FormLabel>Rules</FormLabel>
                  {form.value.rules.map((rule) => (
                    <FormField name={rule.name}>
                      <FormItem>
                        <FormControl>
                          <Checkbox
                            checked={rule.checked}
                            onUpdate:checked={(check: boolean) => {
                              if (check) {
                                form.value.rules.forEach((r) => {
                                  if (
                                    r.id === rule.id &&
                                    r.color === rule.color &&
                                    r.regexp === rule.regexp &&
                                    r.name === rule.name
                                  ) {
                                    r.checked = true;
                                  }
                                });

                                form.value.rulesToCreate.push(rule);

                                form.value.rulesToDelete =
                                  form.value.rulesToDelete.filter(
                                    (r) => r.id !== rule.id
                                  );
                              } else {
                                form.value.rules.forEach((r) => {
                                  if (
                                    r.id === rule.id &&
                                    r.color === rule.color &&
                                    r.regexp === rule.regexp &&
                                    r.name === rule.name
                                  ) {
                                    r.checked = false;
                                  }
                                });

                                form.value.rulesToDelete.push(rule);

                                form.value.rulesToCreate =
                                  form.value.rulesToCreate.filter(
                                    (r) => r.id !== rule.id
                                  );
                              }
                            }}
                          ></Checkbox>
                        </FormControl>
                        <FormLabel>
                          <Badge
                            class="text-xs"
                            style={`background-color: ${rule.color};`}
                          >
                            {rule.name}
                          </Badge>
                        </FormLabel>
                      </FormItem>
                    </FormField>
                  ))}
                  <FormDescription></FormDescription>
                  <FormMessage />
                </FormItem>
              </FormField>
            </form>
          </Form>
          <DialogFooter>
            <DialogClose asChild>
              <Button onClick={handleSubmit} disabled={disabledSubmit.value}>
                {actionText.value}
              </Button>
              {actionText.value === "Update" && (
                <Button variant={"destructive"} onClick={handleDelete}>
                  Delete
                </Button>
              )}
            </DialogClose>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    );
  },
});
