export const readStoredJson = <T>(key: string): T | null => {
  const raw = window.localStorage.getItem(key)
  if (!raw) {
    return null
  }

  try {
    return JSON.parse(raw) as T
  } catch (_error) {
    window.localStorage.removeItem(key)
    return null
  }
}

export const writeStoredJson = (key: string, value: unknown | null) => {
  if (value === null) {
    window.localStorage.removeItem(key)
    return
  }

  window.localStorage.setItem(key, JSON.stringify(value))
}
