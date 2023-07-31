import { useEffect, useState } from "react";
import { merge } from 'lodash'
import { Store } from "tauri-plugin-store-api";
import { SETTINGS_FILE } from "../types";
export const store = new Store(SETTINGS_FILE)

export default function useStorage<T>(key: string, defaultValue: T): [T, (value: T) => void] {
  const [value, setValue] = useState(defaultValue);

  useEffect(() => {
    (async () => {
      const item = await store.get<T>(key)
      if (!item) await store.set(key, defaultValue)
      setValue(item ?? defaultValue)
    })()
  }, [])

  const setValueWrap = (value: T) => {
    try {
      (async () => {
        const currentSettings = await store.get<T>(key)
        const promise = store.set(key, merge(defaultValue, currentSettings, value))
        await store.save()
        setValue(value);
        return promise
      })()
    } catch (e) { console.error(e) }
  };

  return [value, setValueWrap];
}