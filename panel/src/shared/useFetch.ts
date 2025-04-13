import {useCallback} from "react";
import {useToast} from "@shadcn/use-toast.ts";
import {fetchWithAuth} from "@c/Login/LoginPage.tsx";
import {usePublicConfig} from "@s/PublicConfigProvider.tsx";

export default function useFetch() {
  const {serverBaseUrl} = usePublicConfig();
  const {toast} = useToast();
  const fetch = useCallback((urlPath: string, errorToast: string, successToast?: string, init?: RequestInit) => {
    return fetchWithAuth(serverBaseUrl, urlPath, init)
      .then(response => response.json())
      .then(value => {
        if (successToast) {
          toast({className: "toast toast-success", title: successToast});
        }
        return value;
      })
      .catch(reason => toast({
        className: "toast toast-failure",
        title: errorToast,
        description: reason.toString()
      }))
  }, [toast]);

  return {sendData: fetch};
}
