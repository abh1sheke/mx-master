export type MXRecord = {
  domain: string,
  ttl: number,
  priority: number,
  target: string,
}

export type ErrorRecord = {
  domain: string,
  reason: string,
}

