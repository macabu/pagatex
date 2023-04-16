defmodule Pagatex do
  use Rustler, otp_app: :pagatex, crate: "pagatex"

  def who_pays_whom(_payments), do: :erlang.nif_error(:nif_not_loaded)
end
