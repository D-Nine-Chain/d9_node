- store referral relationship
- create off chain worker in Hooks for reading block events
- event for referral relationship recorded
- write functions for off chain inteeration of this pallet
  - get parent
  - get descendants
  - get depth
  - is predecessor
  - is antecedent
  - get account depth
- write errors pallet
- write function to get referral data for chain extensions (put that chain extension in seperate file near runtime)

question:

- are weights only for transactions that have origins?

  - review why weights are used
  - review the pallet api
  - review about calls that dont have a call index. do they need weights?

- declared runtime api implement runtime api into runtime
  use it for informational requests
