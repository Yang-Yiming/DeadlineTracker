//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.
//!
//!
//! The [`Home`] and [`Blog`] components will be rendered when the current route is [`Route::Home`] or [`Route::Blog`] respectively.
//!
//!
//! The [`Navbar`] component will be rendered on all pages of our app since every page is under the layout. The layout defines
//! a common wrapper around all child routes.

mod blog;
pub use blog::Blog;

mod home;
pub use home::Home;

mod navbar;
pub use navbar::Navbar;

mod deadline_item_view;
#[allow(unused_imports)]
pub use deadline_item_view::DeadlineItemView;

mod edit_deadline_view;
pub use edit_deadline_view::EditDeadlineView;

mod deadline_list_view;
pub use deadline_list_view::DeadlineListView;

mod calendar_view;
pub use calendar_view::CalendarView;
