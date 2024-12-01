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

import { useRuleSidebar } from "./use-rule-dialog";

import { Button } from "~/components/ui/button";
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
import axios from "axios";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select";

const getForm = (input?: Rule | null) => {
  return {
    id: input?.id || 0,
    name: input?.name || "",
    packetDirection: input?.packetDirection || "",
    regexp: input?.regexp || "",
    color: input?.color || "",

    // FIXME: hardcoded
    allPacketDirections: ["IN", "OUT"],
  };
};

export const RuleDialog = defineComponent({
  name: "RuleDialog",
  props: {
    data: {
      type: Object as () => Rule,
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
    const ruleSidebar = useRuleSidebar();

    const isSubmitting = ref(false);
    const isOpen = ref(true);

    const isNew = computed(() => {
      return !props.data?.id;
    });

    const form = ref(getForm(props.data));

    const isFormDataValid = computed(() => {
      return (
        !!form.value.name.length &&
        !!form.value.color &&
        !!form.value.packetDirection &&
        !!form.value.regexp
      );
    });

    const disabledSubmit = computed(() => {
      return !isFormDataValid.value || isSubmitting.value;
    });

    const actionText = computed(() => {
      return isNew.value ? "Create" : "Update";
    });

    const getRuleToUpsert = () => {
      return {
        id: form.value?.id,
        name: form.value?.name,
        packet_direction: form.value?.packetDirection,
        regexp: form.value?.regexp,
        color: form.value?.color,
      };
    };

    const handleSubmit = async () => {
      if (!isFormDataValid.value) return;

      try {
        isSubmitting.value = true;

        if (isNew.value) {
          await ruleSidebar.createRule(getRuleToUpsert());
        } else {
          await ruleSidebar.updateRule(getRuleToUpsert());
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

        await ruleSidebar.deleteRule(form.value?.id);

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
            <DialogTitle>{actionText.value} rule</DialogTitle>
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
                      placeholder="rule name"
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
              <FormField name="regexp">
                <FormItem>
                  <FormLabel>Regexp</FormLabel>
                  <FormControl>
                    <Input
                      type="text"
                      placeholder="regexp"
                      onChange={(event: any) => {
                        form.value.regexp = String(event.target.value);
                      }}
                      modelValue={form.value?.regexp || ""}
                    />
                  </FormControl>
                  <FormDescription></FormDescription>
                  <FormMessage />
                </FormItem>
              </FormField>
              <FormField name="color">
                <FormItem>
                  <FormLabel>Color</FormLabel>
                  <FormControl>
                    <Input
                      type="color"
                      placeholder="color"
                      onChange={(event: any) => {
                        form.value.color = String(event.target.value);
                      }}
                      modelValue={form.value?.color || ""}
                    />
                  </FormControl>
                  <FormDescription></FormDescription>
                  <FormMessage />
                </FormItem>
              </FormField>
              <FormField name="packet_direction">
                <FormItem>
                  <FormLabel>Packet direction</FormLabel>
                  <Select
                    onUpdate:modelValue={(direction: string) => {
                      form.value.packetDirection = direction;
                    }}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue
                          placeholder={
                            form.value?.packetDirection ||
                            "select packet direction"
                          }
                        ></SelectValue>
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      <SelectGroup>
                        {form.value.allPacketDirections.map((d) => (
                          <SelectItem value={d}>{d}</SelectItem>
                        ))}
                      </SelectGroup>
                    </SelectContent>
                  </Select>
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
