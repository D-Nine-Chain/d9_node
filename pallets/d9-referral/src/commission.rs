trait CommissionCalculation<T: Config<I>, I: 'static> {
	fn parent_commission(
		amount: BalanceOf<T>,
		percentage: u32
	) -> Result<BalanceOf<T>, DispatchError>;

	fn non_parent_ancestor_commission(
		amount: BalanceOf<T>,
		percentage: u32
	) -> Result<BalanceOf<T>, DispatchError>;
}
