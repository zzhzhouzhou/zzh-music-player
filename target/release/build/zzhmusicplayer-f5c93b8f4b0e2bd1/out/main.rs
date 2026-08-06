mod slint_generatedMainWindow {
     # ! [allow (non_snake_case , non_camel_case_types)] # ! [allow (unused_braces , unused_parens)] # ! [allow (clippy :: all , clippy :: pedantic , clippy :: nursery)] # ! [allow (unknown_lints , if_let_rescope , tail_expr_drop_order)] use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_17_1 = slint :: VersionCheck_1_17_1 ;
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerUIState {
         r#about_open : sp :: Property < bool > , r#always_on_top : sp :: Property < bool > , r#bg_image : sp :: Property < sp :: Image > , r#drag_fraction : sp :: Property < f32 > , r#dragging : sp :: Property < bool > , r#duration : sp :: Property < f32 > , r#mode_showing : sp :: Property < bool > , r#mode_text : sp :: Property < sp :: SharedString > , r#particle_time : sp :: Property < f32 > , r#playing : sp :: Property < bool > , r#playlist : sp :: Property < sp :: ModelRc < sp :: SharedString > > , r#playlist_current : sp :: Property < i32 > , r#playlist_open : sp :: Property < bool > , r#position : sp :: Property < f32 > , r#seek_pending : sp :: Property < bool > , r#theme_color : sp :: Property < sp :: Color > , r#toolbar_hovered : sp :: Property < bool > , r#track_artist : sp :: Property < sp :: SharedString > , r#track_title : sp :: Property < sp :: SharedString > , r#volume : sp :: Property < f32 > , r#volume_popup_open : sp :: Property < bool > , r#volume_text : sp :: Property < sp :: SharedString > , r#wave_bg_image : sp :: Property < sp :: Image > , r#wave_fg_image : sp :: Property < sp :: Image > , r#clear_playlist : sp :: Callback < () , () > , r#close_window : sp :: Callback < () , () > , r#cycle_mode : sp :: Callback < () , () > , r#minimize_window : sp :: Callback < () , () > , r#next : sp :: Callback < () , () > , r#play_at : sp :: Callback < (i32 ,) , () > , r#previous : sp :: Callback < () , () > , r#remove_track : sp :: Callback < (i32 ,) , () > , r#seek_relative : sp :: Callback < (f32 ,) , () > , r#seek_requested : sp :: Callback < (f32 ,) , () > , r#set_volume : sp :: Callback < (f32 ,) , () > , r#toggle_pin : sp :: Callback < () , () > , r#toggle_play : sp :: Callback < () , () > , r#toggle_playlist : sp :: Callback < () , () > , r#toggle_volume_popup : sp :: Callback < () , () > , r#window_drag_down : sp :: Callback < (f32 , f32 ,) , () > , r#window_drag_move : sp :: Callback < (f32 , f32 ,) , () > , r#window_drag_up : sp :: Callback < () , () > , callback_tracker_clear_playlist : sp :: Property < () > , callback_tracker_close_window : sp :: Property < () > , callback_tracker_cycle_mode : sp :: Property < () > , callback_tracker_minimize_window : sp :: Property < () > , callback_tracker_next : sp :: Property < () > , callback_tracker_play_at : sp :: Property < () > , callback_tracker_previous : sp :: Property < () > , callback_tracker_remove_track : sp :: Property < () > , callback_tracker_seek_relative : sp :: Property < () > , callback_tracker_seek_requested : sp :: Property < () > , callback_tracker_set_volume : sp :: Property < () > , callback_tracker_toggle_pin : sp :: Property < () > , callback_tracker_toggle_play : sp :: Property < () > , callback_tracker_toggle_playlist : sp :: Property < () > , callback_tracker_toggle_volume_popup : sp :: Property < () > , callback_tracker_window_drag_down : sp :: Property < () > , callback_tracker_window_drag_move : sp :: Property < () > , callback_tracker_window_drag_up : sp :: Property < () > , globals : sp :: OnceCell < sp :: Weak < SharedGlobals >> , }
     impl InnerUIState {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , globals : & sp :: Rc < SharedGlobals >) {
             # ! [allow (unused)] let _ = self . globals . set (sp :: Rc :: downgrade (globals)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#about_open () }
             . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#always_on_top () }
             . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#drag_fraction () }
             . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#dragging () }
             . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#duration () }
             . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#mode_showing () }
             . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#mode_text () }
             . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("顺序播放")) as sp :: SharedString }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#particle_time () }
             . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#playing () }
             . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerUIState :: FIELD_OFFSETS . r#playlist () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: ModelRc :: new (sp :: VecModel :: < sp :: SharedString > :: from (sp :: vec ! []))) as _ }
                ) ;
                 }
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#playlist_current () }
             . apply_pin (_self) . set ({
                 (((- 1f64) as i32)) as i32 }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#playlist_open () }
             . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#position () }
             . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#seek_pending () }
             . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
             . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded ((4284139770f64) as u32)) as sp :: Color }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#toolbar_hovered () }
             . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#track_artist () }
             . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#track_title () }
             . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#volume () }
             . apply_pin (_self) . set ({
                 (1f64) as f32 }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#volume_popup_open () }
             . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#volume_text () }
             . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             }
         }
     # [allow (unused)] pub struct r#UIState < 'a > (:: core :: pin :: Pin < sp :: Rc < InnerUIState >> , :: core :: marker :: PhantomData < & 'a InnerUIState >) ;
     impl < 'a > r#UIState < 'a > {
         # [allow (dead_code)] pub fn get_about_open (& self) -> bool {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#about_open () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_about_open (& self , value : bool) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#about_open () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_always_on_top (& self) -> bool {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#always_on_top () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_always_on_top (& self , value : bool) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#always_on_top () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_bg_image (& self) -> sp :: Image {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#bg_image () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_bg_image (& self , value : sp :: Image) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#bg_image () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_clear_playlist (& self ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#clear_playlist () }
             . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_clear_playlist (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#clear_playlist () }
             . apply_pin (_self) . set_handler (move | args | f ()) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_clear_playlist () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn invoke_close_window (& self ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#close_window () }
             . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_close_window (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#close_window () }
             . apply_pin (_self) . set_handler (move | args | f ()) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_close_window () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn invoke_cycle_mode (& self ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#cycle_mode () }
             . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_cycle_mode (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#cycle_mode () }
             . apply_pin (_self) . set_handler (move | args | f ()) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_cycle_mode () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn get_drag_fraction (& self) -> f32 {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#drag_fraction () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_drag_fraction (& self , value : f32) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#drag_fraction () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_dragging (& self) -> bool {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#dragging () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_dragging (& self , value : bool) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#dragging () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_duration (& self) -> f32 {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#duration () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_duration (& self , value : f32) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#duration () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_minimize_window (& self ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#minimize_window () }
             . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_minimize_window (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#minimize_window () }
             . apply_pin (_self) . set_handler (move | args | f ()) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_minimize_window () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn get_mode_showing (& self) -> bool {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#mode_showing () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_mode_showing (& self , value : bool) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#mode_showing () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_mode_text (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#mode_text () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_mode_text (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#mode_text () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_next (& self ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#next () }
             . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_next (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#next () }
             . apply_pin (_self) . set_handler (move | args | f ()) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_next () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn get_particle_time (& self) -> f32 {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#particle_time () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_particle_time (& self , value : f32) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#particle_time () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_play_at (& self , arg_0 : i32 ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#play_at () }
             . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_play_at (& self , mut f : impl FnMut (i32) -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#play_at () }
             . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_play_at () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn get_playing (& self) -> bool {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#playing () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_playing (& self , value : bool) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#playing () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_playlist (& self) -> sp :: ModelRc < sp :: SharedString > {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#playlist () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_playlist (& self , value : sp :: ModelRc < sp :: SharedString >) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#playlist () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_playlist_current (& self) -> i32 {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#playlist_current () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_playlist_current (& self , value : i32) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#playlist_current () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_playlist_open (& self) -> bool {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#playlist_open () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_playlist_open (& self , value : bool) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#playlist_open () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_position (& self) -> f32 {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#position () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_position (& self , value : f32) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#position () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_previous (& self ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#previous () }
             . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_previous (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#previous () }
             . apply_pin (_self) . set_handler (move | args | f ()) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_previous () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn invoke_remove_track (& self , arg_0 : i32 ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#remove_track () }
             . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_remove_track (& self , mut f : impl FnMut (i32) -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#remove_track () }
             . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_remove_track () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn get_seek_pending (& self) -> bool {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#seek_pending () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_seek_pending (& self , value : bool) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#seek_pending () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_seek_relative (& self , arg_0 : f32 ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#seek_relative () }
             . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_seek_relative (& self , mut f : impl FnMut (f32) -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#seek_relative () }
             . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_seek_relative () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn invoke_seek_requested (& self , arg_0 : f32 ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#seek_requested () }
             . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_seek_requested (& self , mut f : impl FnMut (f32) -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#seek_requested () }
             . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_seek_requested () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn invoke_set_volume (& self , arg_0 : f32 ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#set_volume () }
             . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_set_volume (& self , mut f : impl FnMut (f32) -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#set_volume () }
             . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_set_volume () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn get_theme_color (& self) -> sp :: Color {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_theme_color (& self , value : sp :: Color) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_toggle_pin (& self ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#toggle_pin () }
             . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_toggle_pin (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#toggle_pin () }
             . apply_pin (_self) . set_handler (move | args | f ()) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_toggle_pin () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn invoke_toggle_play (& self ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#toggle_play () }
             . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_toggle_play (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#toggle_play () }
             . apply_pin (_self) . set_handler (move | args | f ()) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_toggle_play () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn invoke_toggle_playlist (& self ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#toggle_playlist () }
             . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_toggle_playlist (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#toggle_playlist () }
             . apply_pin (_self) . set_handler (move | args | f ()) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_toggle_playlist () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn invoke_toggle_volume_popup (& self ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#toggle_volume_popup () }
             . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_toggle_volume_popup (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#toggle_volume_popup () }
             . apply_pin (_self) . set_handler (move | args | f ()) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_toggle_volume_popup () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn get_toolbar_hovered (& self) -> bool {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#toolbar_hovered () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_toolbar_hovered (& self , value : bool) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#toolbar_hovered () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_track_artist (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#track_artist () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_track_artist (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#track_artist () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_track_title (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#track_title () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_track_title (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#track_title () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_volume (& self) -> f32 {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#volume () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_volume (& self , value : f32) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#volume () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_volume_popup_open (& self) -> bool {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#volume_popup_open () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_volume_popup_open (& self , value : bool) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#volume_popup_open () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_volume_text (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#volume_text () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_volume_text (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#volume_text () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_wave_bg_image (& self) -> sp :: Image {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#wave_bg_image () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_wave_bg_image (& self , value : sp :: Image) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#wave_bg_image () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_wave_fg_image (& self) -> sp :: Image {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#wave_fg_image () }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_wave_fg_image (& self , value : sp :: Image) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#wave_fg_image () }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_window_drag_down (& self , arg_0 : f32 , arg_1 : f32 ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#window_drag_down () }
             . apply_pin (_self) . call (& (arg_0 , arg_1 ,)) }
         # [allow (dead_code)] pub fn on_window_drag_down (& self , mut f : impl FnMut (f32 , f32) -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#window_drag_down () }
             . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone () , args . 1 . clone ())) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_window_drag_down () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn invoke_window_drag_move (& self , arg_0 : f32 , arg_1 : f32 ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#window_drag_move () }
             . apply_pin (_self) . call (& (arg_0 , arg_1 ,)) }
         # [allow (dead_code)] pub fn on_window_drag_move (& self , mut f : impl FnMut (f32 , f32) -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#window_drag_move () }
             . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone () , args . 1 . clone ())) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_window_drag_move () }
             . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn invoke_window_drag_up (& self ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . r#window_drag_up () }
             . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_window_drag_up (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerUIState :: FIELD_OFFSETS . r#window_drag_up () }
             . apply_pin (_self) . set_handler (move | args | f ()) ;
             {
                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_window_drag_up () }
             . apply_pin (_self) . mark_dirty () ;
             }
         }
     impl < 'a > slint :: Global < 'a , r#MainWindow > for r#UIState < 'a > {
         type StaticSelf = r#UIState < 'static > ;
         fn get (component : & 'a r#MainWindow) -> Self {
             Self (component . 0 . globals . get () . unwrap () . global_UIState . clone () , :: core :: marker :: PhantomData :: default ()) }
         fn as_weak (& self) -> slint :: Weak < Self :: StaticSelf > {
             let inner = :: core :: pin :: Pin :: into_inner (self . 0 . clone ()) ;
             slint :: Weak :: new (sp :: Rc :: downgrade (& inner)) }
         }
     impl slint :: StrongHandle for r#UIState < 'static > {
         type WeakInner = sp :: Weak < InnerUIState > ;
         fn upgrade_from_weak_inner (inner : & Self :: WeakInner) -> :: core :: option :: Option < Self > {
             let inner = :: core :: pin :: Pin :: new (inner . upgrade () ?) ;
             :: core :: option :: Option :: Some (Self (inner , :: core :: marker :: PhantomData :: default ())) }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerWaveformArea_root_1 {
         r#root_1 : sp :: r#Rectangle , r#image_2 : sp :: r#ImageItem , r#rectangle_3 : sp :: r#Empty , r#_clip_4 : sp :: r#Clip , r#image_5 : sp :: r#ImageItem , r#_Opacity_6 : sp :: r#Opacity , r#rectangle_7 : sp :: r#Rectangle , r#rectangle_8 : sp :: r#Rectangle , r#toucharea_12 : sp :: r#TouchArea , r#root_1_height : sp :: Property < sp :: LogicalLength > , r#root_1_image_2_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_image_2_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_image_5_y : sp :: Property < sp :: LogicalLength > , r#root_1_progress_frac : sp :: Property < f32 > , r#root_1_rectangle_3_width : sp :: Property < sp :: LogicalLength > , r#root_1_rectangle_7_x : sp :: Property < sp :: LogicalLength > , r#root_1_rectangle_8_x : sp :: Property < sp :: LogicalLength > , r#root_1_toucharea_12_preview : sp :: Property < f32 > , r#root_1_width : sp :: Property < sp :: LogicalLength > , r#root_1_x : sp :: Property < sp :: LogicalLength > , r#root_1_y : sp :: Property < sp :: LogicalLength > , repeater0 : sp :: Repeater < InnerComponent__Opacity_9 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerWaveformArea_root_1 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerWaveformArea_root_1 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (sp :: VecModel :: < f32 > :: from (sp :: vec ! [(0f64) . clone () as _ , (0.23f64) . clone () as _ , (0.11f64) . clone () as _ , (0.47f64) . clone () as _ , (0.05f64) . clone () as _ , (0.62f64) . clone () as _ , (0.34f64) . clone () as _ , (0.81f64) . clone () as _]))) as _ }
                 }
            ) ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#Rectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_image_2_preferred_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_2 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_image_2_preferred_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_2 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_image_5_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((((((1f64) . clone ()) as f64) * ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) - ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_rectangle_3_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) * ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_progress_frac ()) . apply_pin (_self) . get ()) . clone ()) as f64)) as sp :: Coord) . max (0f64 as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_rectangle_7_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) * ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_progress_frac ()) . apply_pin (_self) . get ()) . clone ()) as f64))) . clone ()) as f64) - (((1f64) . clone ()) as f64)) as sp :: Coord) . max (0f64 as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_rectangle_8_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) * ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_progress_frac ()) . apply_pin (_self) . get ()) . clone ()) as f64))) . clone ()) as f64) - (((38f64) . clone ()) as f64)) as sp :: Coord) . max (0f64 as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_toucharea_12_preview ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#toucharea_12 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) / ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as f32) . max (0f64 as f32) as f32) . min (1f64 as f32)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_2 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1f64) . clone ()) as f64) * ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_2 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_2 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerUIState :: FIELD_OFFSETS . r#wave_bg_image () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_2 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1f64) . clone ()) as f64) * ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#_clip_4 () + sp :: r#Clip :: FIELD_OFFSETS . r#clip ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_5 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_5 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Fill) as sp :: r#ImageFit }
            ) ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_5 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_5 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerUIState :: FIELD_OFFSETS . r#wave_fg_image () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_5 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#_Opacity_6 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) . set ({
                 (0.95f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#rectangle_7 () + sp :: r#Rectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#rectangle_8 () + sp :: r#Rectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_UIState_theme_color = ({
                             * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                         . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) . clone () ;
                         slint :: Brush :: RadialGradient (sp :: RadialGradientBrush :: new_circle ([sp :: GradientStop {
                             color : (slint :: Brush :: SolidColor (r#tmp_UIState_theme_color)) . clone () . with_alpha ((0.38f64) . clone () as f32) . color () , position : 0f64 as _ }
                         , sp :: GradientStop {
                             color : (slint :: Brush :: SolidColor (r#tmp_UIState_theme_color)) . clone () . with_alpha ((0.12f64) . clone () as f32) . color () , position : 0.45f64 as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((0f64) as u32) , position : 1f64 as _ }
                        ])) }
                    ) as _ }
                ) ;
                 }
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#toucharea_12 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#toucharea_12 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#toucharea_12 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((((args . 0 . clone ()) . r#kind) . clone ())) == (((sp :: r#PointerEventKind :: r#Down) . clone ())) {
                                 ({
                                     {
                                         * & InnerUIState :: FIELD_OFFSETS . r#dragging () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . set (true as _) ;
                                     {
                                         * & InnerUIState :: FIELD_OFFSETS . r#drag_fraction () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . set ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_toucharea_12_preview ()) . apply_pin (_self) . get () as _) }
                                ) ;
                                 }
                             else {
                                 if ((((args . 0 . clone ()) . r#kind) . clone ())) == (((sp :: r#PointerEventKind :: r#Move) . clone ())) {
                                     ({
                                         if {
                                             * & InnerUIState :: FIELD_OFFSETS . r#dragging () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () {
                                             ({
                                                 {
                                                     * & InnerUIState :: FIELD_OFFSETS . r#drag_fraction () }
                                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . set ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_toucharea_12_preview ()) . apply_pin (_self) . get () as _) }
                                            ) ;
                                             }
                                         else {
                                             {
                                                 }
                                             }
                                         }
                                    ) ;
                                     }
                                 else {
                                     if ((((args . 0 . clone ()) . r#kind) . clone ())) == (((sp :: r#PointerEventKind :: r#Up) . clone ())) {
                                         ({
                                             if {
                                                 * & InnerUIState :: FIELD_OFFSETS . r#dragging () }
                                             . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () {
                                                 ({
                                                     {
                                                         {
                                                             * & InnerUIState :: FIELD_OFFSETS . callback_tracker_seek_requested () }
                                                         . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                                         {
                                                             * & InnerUIState :: FIELD_OFFSETS . r#seek_requested () }
                                                         . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& (((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_toucharea_12_preview ()) . apply_pin (_self) . get ()) . clone () as _ ,)) ;
                                                         }
                                                     ;
                                                     {
                                                         * & InnerUIState :: FIELD_OFFSETS . r#seek_pending () }
                                                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . set (true as _) }
                                                ) ;
                                                 }
                                             else {
                                                 {
                                                     }
                                                 }
                                             }
                                        ) ;
                                         }
                                     else {
                                         {
                                             }
                                         }
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#Rectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_2 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_2 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_2 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#_clip_4 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#_clip_4 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#_clip_4 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#_clip_4 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#_clip_4 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_5 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_5 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_5 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#toucharea_12 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#toucharea_12 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerWaveformArea_root_1 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             {
                 _changed |= InnerWaveformArea_root_1 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent__Opacity_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                 }
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ((((({
                         let r#layout_info_0 = ({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone () ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_0) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_0) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((r#layout_info_0) . r#min) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_0) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info_0) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_image_2_preferred_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#stretch = (0f64) . clone () as _ ;
                         the_struct }
                    ) . clone ())))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => _self . r#fn_layoutinfo_v_with_constraint ((((({
                     let r#layout_info_0 = ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = (0f64) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                    ) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info_0) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info_0) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((r#layout_info_0) . r#min) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info_0) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info_0) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                     }
                ) . clone ())) + ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_image_2_preferred_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                     the_struct . r#stretch = (0f64) . clone () as _ ;
                     the_struct }
                ) . clone ()))) . r#preferred as _) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerWaveformArea_root_1 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 1u32 => (((((((1f64) . clone ()) as f64) * ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 2u32 => (((((((1f64) . clone ()) as f64) * ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , (((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_rectangle_3_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 3u32 => (((((((1f64) . clone ()) as f64) * ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((2f64) . clone ()) . clone () as sp :: Coord , (((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_rectangle_7_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 4u32 => (((76f64) . clone ()) . clone () as sp :: Coord , ((76f64) . clone ()) . clone () as sp :: Coord , (((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_rectangle_8_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((((((((((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) / (((2f64) . clone ()) as f64))) . clone ()) as f64) - (((38f64) . clone ()) as f64)) as sp :: Coord) . max (0f64 as sp :: Coord)) . clone ()) . clone () as sp :: Coord ,) , 6u32 => ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 7u32 => (((((((1f64) . clone ()) as f64) * ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , (((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_rectangle_3_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 8u32 => ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , (((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_image_5_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 9u32 => (((((((1f64) . clone ()) as f64) * ((((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((2f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => sp :: r#AccessibleRole :: r#Image , 8u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ((((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = (0f64) . clone () as _ ;
                 the_struct . r#stretch = (1f64) . clone () as _ ;
                 the_struct }
            ) . clone ())) + ((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = ((InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_image_2_preferred_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct . r#stretch = (0f64) . clone () as _ ;
                 the_struct }
            ) . clone ())))) as _ }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent__Opacity_9 {
         r#_Opacity_9 : sp :: r#Opacity , r#rectangle_10 : sp :: r#BasicBorderRectangle , r#model_data : sp :: Property < f32 > , r#model_index : sp :: Property < i32 > , r#_Opacity_9_rectangle_10_age : sp :: Property < f32 > , r#_Opacity_9_rectangle_10_height : sp :: Property < sp :: LogicalLength > , r#_Opacity_9_rectangle_10_width : sp :: Property < sp :: LogicalLength > , r#_Opacity_9_rectangle_10_x : sp :: Property < sp :: LogicalLength > , r#_Opacity_9_rectangle_10_y : sp :: Property < sp :: LogicalLength > , r#_Opacity_9_rectangle_10_yoff : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent__Opacity_9 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerWaveformArea_root_1 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent__Opacity_9 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((((1f64) . clone ()) as f64) - ((((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_age ()) . apply_pin (_self) . get ()) . clone ()) as f64))) . clone () as f64) . powf ((((((1.4f64) . clone ()) as f64) + (((((((sp :: Euclid :: rem_euclid (& (((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((3f64) . clone () as f64))) . clone ()) as f64) * (((0.4f64) . clone ()) as f64))) . clone ()) as f64))) . clone () as f64)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_age ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Euclid :: rem_euclid (& (((((({
                         * & InnerUIState :: FIELD_OFFSETS . r#particle_time () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) . clone ()) as f64) + ((((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#model_data ()) . apply_pin (_self) . get ()) . clone ()) as f64))) . clone () as f64) , & ((1f64) . clone () as f64))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((2f64 as sp :: Coord) . max (((((3f64) . clone ()) as f64) - ((((((((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_age ()) . apply_pin (_self) . get ()) . clone ()) as f64) * (((((((1f64) . clone ()) as f64) + (((((((sp :: Euclid :: rem_euclid (& (((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((2f64) . clone () as f64))) . clone ()) as f64) * (((1f64) . clone ()) as f64))) . clone ()) as f64))) . clone ()) as f64))) . clone ()) as f64)) as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((2f64 as sp :: Coord) . max (((((3f64) . clone ()) as f64) - ((((((((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_age ()) . apply_pin (_self) . get ()) . clone ()) as f64) * (((((((1f64) . clone ()) as f64) + (((((((sp :: Euclid :: rem_euclid (& (((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((2f64) . clone () as f64))) . clone ()) as f64) * (((1f64) . clone ()) as f64))) . clone ()) as f64))) . clone ()) as f64))) . clone ()) as f64)) as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) * (((_self . parent . upgrade () . as_ref () . map (| x | (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_progress_frac ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) . clone ()) as f64))) . clone ()) as f64) - ((((((((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_age ()) . apply_pin (_self) . get ()) . clone ()) as f64) * (((((((68f64) . clone ()) as f64) + (((((((sp :: Euclid :: rem_euclid (& (((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((3f64) . clone () as f64))) . clone ()) as f64) * (((20f64) . clone ()) as f64))) . clone ()) as f64))) . clone ()) as f64))) . clone ()) as f64)) as sp :: Coord) . max (0f64 as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_root_1_height = (_self . parent . upgrade () . as_ref () . map (| x | (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone () ;
                         (((((((((((((((((r#tmp_root_1_height) . clone ()) as f64) / (((2f64) . clone ()) as f64))) . clone ()) as f64) + ((((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_yoff ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) + ((((((((((((((((((((((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_age ()) . apply_pin (_self) . get ()) . clone ()) as f64) * (((360f64) . clone ()) as f64))) . clone ()) as f64) + ((((((((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get ()) . clone ()) as f64) * (((77f64) . clone ()) as f64))) . clone ()) as f64))) . clone () as f64) . to_radians () . sin ()) . clone ()) as f64) * (((r#tmp_root_1_height) . clone ()) as f64))) . clone ()) as f64) * (((0.05f64) . clone ()) as f64))) . clone ()) as f64))) . clone ()) as f64) - (((1.5f64) . clone ()) as f64)) as sp :: Coord) . max (0f64 as sp :: Coord) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_yoff ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((((((((((((((sp :: Euclid :: rem_euclid (& (((((((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get ()) . clone ()) as f64) * (((31f64) . clone ()) as f64))) . clone () as f64) , & ((7f64) . clone () as f64))) . clone ()) as f64) * (((2f64) . clone ()) as f64))) . clone ()) as f64) + (((sp :: Euclid :: rem_euclid (& (((((((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get ()) . clone ()) as f64) * (((17f64) . clone ()) as f64))) . clone () as f64) , & ((5f64) . clone () as f64))) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) * (((_self . parent . upgrade () . as_ref () . map (| x | (InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((30f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#rectangle_10 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4294967295f64) as u32))) as slint :: Brush }
            ) ;
             (InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#rectangle_10 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1.5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#rectangle_10 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#rectangle_10 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#rectangle_10 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#rectangle_10 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                 , sp :: Orientation :: Vertical => {
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             (InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get () as usize }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9_rectangle_10_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent__Opacity_9 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerWaveformArea_root_1 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerWaveformArea_root_1 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent__Opacity_9 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#_Opacity_9 ()) , sp :: VOffset :: new (InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#rectangle_10 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent__Opacity_9) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent__Opacity_9 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent__Opacity_9 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent__Opacity_9 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent__Opacity_9 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 5u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             self . ensure_instantiated () }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent__Opacity_9 {
         type Data = f32 ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             (InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . set (_index as _) ;
             (InnerComponent__Opacity_9 :: FIELD_OFFSETS . r#model_data ()) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent__Opacity_9 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerVolumeBar_root_13 {
         r#root_13 : sp :: r#BasicBorderRectangle , r#rectangle_14 : sp :: r#BasicBorderRectangle , r#toucharea_15 : sp :: r#TouchArea , r#root_13_rectangle_14_width : sp :: Property < sp :: LogicalLength > , r#root_13_volume : sp :: Property < f32 > , r#root_13_width : sp :: Property < sp :: LogicalLength > , r#root_13_x : sp :: Property < sp :: LogicalLength > , r#root_13_y : sp :: Property < sp :: LogicalLength > , r#root_13_changed : sp :: Callback < (f32 ,) , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerVolumeBar_root_13 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerVolumeBar_root_13 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((486539263f64) as u32))) as slint :: Brush }
            ) ;
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_rectangle_14_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((4f64) . clone ()) as f64))) . clone ()) as f64) * ((((0f64 as f32) . max (((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_volume ()) . apply_pin (_self) . get () as f32) . min (1f64 as f32) as f32)) . clone ()) as f64)) as sp :: Coord) . max (0f64 as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_volume ()) . apply_pin (_self) . set ({
                 (1f64) as f32 }
            ) ;
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (110f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#rectangle_14 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#rectangle_14 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (6f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#toucharea_15 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#toucharea_15 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#toucharea_15 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((((((((args . 0 . clone ()) . r#kind) . clone ())) == (((sp :: r#PointerEventKind :: r#Down) . clone ())))) . clone ())) || ((((((((((((args . 0 . clone ()) . r#kind) . clone ())) == (((sp :: r#PointerEventKind :: r#Move) . clone ())))) . clone ())) && ((((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#toucharea_15 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())) {
                                 ({
                                     {
                                         (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_changed ()) . apply_pin (_self) . call (& (((0f64 as f32) . max (((((((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#toucharea_15 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) / ((((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as f32) . min (1f64 as f32) as f32)) . clone () as _ ,)) ;
                                         }
                                     }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#rectangle_14 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#rectangle_14 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#rectangle_14 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#toucharea_15 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#toucharea_15 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (sp :: Item :: layout_info ((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (sp :: Item :: layout_info ((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (16f64) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (16f64) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((16f64) . clone ()) . clone () as sp :: Coord , (((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 1u32 => (((12f64) . clone ()) . clone () as sp :: Coord , (((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_rectangle_14_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((2f64) . clone ()) . clone () as sp :: Coord , ((2f64) . clone ()) . clone () as sp :: Coord ,) , 2u32 => (((16f64) . clone ()) . clone () as sp :: Coord , (((InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerPlaylistRow_root_16 {
         r#root_16 : sp :: r#BasicBorderRectangle , r#text_17 : sp :: r#ComplexText , r#toucharea_18 : sp :: r#TouchArea , r#image_19 : sp :: r#ImageItem , r#toucharea_20 : sp :: r#TouchArea , r#root_16_index : sp :: Property < i32 > , r#root_16_text : sp :: Property < sp :: SharedString > , r#root_16_width : sp :: Property < sp :: LogicalLength > , r#root_16_y : sp :: Property < sp :: LogicalLength > , r#root_16_clicked : sp :: Callback < () , () > , r#root_16_remove : sp :: Callback < () , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerPlaylistRow_root_16 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerPlaylistRow_root_16 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_index ()) . apply_pin (_self) . get ()) . clone () as f64) , & (({
                         * & InnerUIState :: FIELD_OFFSETS . r#playlist_current () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) . clone () as f64)) {
                         ((slint :: Brush :: SolidColor ({
                             * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                         . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) . clone () . with_alpha ((0.22f64) . clone () as f32)) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32)) }
                    ) as _ }
                ) ;
                 }
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (6f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if sp :: ApproxEq :: < f64 > :: approx_eq (& (((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_index ()) . apply_pin (_self) . get ()) . clone () as f64) , & (({
                         * & InnerUIState :: FIELD_OFFSETS . r#playlist_current () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) . clone () as f64)) {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4291810525f64) as u32) }
                    )) as _ }
                ) ;
                 }
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((((1f64) . clone ()) as f64) * (((26f64) . clone ()) as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#overflow ()) . apply_pin (_self) . set ({
                 (sp :: r#TextOverflow :: r#Elide) as sp :: r#TextOverflow }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_text ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((40f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#toucharea_18 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#toucharea_18 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#toucharea_18 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#image_19 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#image_19 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#image_19 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#image_19 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#image_19 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#toucharea_20 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_remove ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#toucharea_20 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#toucharea_20 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_family ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_italic ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#letter_spacing ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#overflow ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke_style ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke_width ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#wrap ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#toucharea_18 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#toucharea_18 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#image_19 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#image_19 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#image_19 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#image_19 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#image_19 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#image_19 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#toucharea_20 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#toucharea_20 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => {
                     let r#layout_info = (sp :: Item :: layout_info ((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (26f64) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (26f64) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((26f64) . clone ()) . clone () as sp :: Coord , (((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , (((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 1u32 => (((((((1f64) . clone ()) as f64) * (((26f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , (((((((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((40f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((12f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 2u32 => (((26f64) . clone ()) . clone () as sp :: Coord , (((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 3u32 => (((14f64) . clone ()) . clone () as sp :: Coord , ((14f64) . clone ()) . clone () as sp :: Coord , (((((((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((30f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((6f64) . clone ()) . clone () as sp :: Coord ,) , 4u32 => (((14f64) . clone ()) . clone () as sp :: Coord , ((14f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => sp :: r#AccessibleRole :: r#Text , 3u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_text ()) . apply_pin (_self) . get ()) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerMainWindow {
         r#root_21 : sp :: r#WindowItem , r#shortcuts_22 : sp :: r#FocusScope , r#keybinding_23 : sp :: r#KeyBinding , r#keybinding_24 : sp :: r#KeyBinding , r#keybinding_25 : sp :: r#KeyBinding , r#toucharea_26 : sp :: r#TouchArea , r#rectangle_27 : sp :: r#BasicBorderRectangle , r#_clip_28 : sp :: r#Clip , r#image_29 : sp :: r#ImageItem , r#text_30 : sp :: r#ComplexText , r#_Opacity_31 : sp :: r#Opacity , r#_Transform_32 : sp :: r#Transform , r#rectangle_33 : sp :: r#BasicBorderRectangle , r#_Opacity_34 : sp :: r#Opacity , r#rectangle_35 : sp :: r#BasicBorderRectangle , r#image_36 : sp :: r#ClippedImage , r#ta_37 : sp :: r#TouchArea , r#_Opacity_38 : sp :: r#Opacity , r#_Transform_39 : sp :: r#Transform , r#rectangle_40 : sp :: r#BasicBorderRectangle , r#_Opacity_41 : sp :: r#Opacity , r#rectangle_42 : sp :: r#BasicBorderRectangle , r#image_43 : sp :: r#ClippedImage , r#ta_44 : sp :: r#TouchArea , r#_Opacity_45 : sp :: r#Opacity , r#_Transform_46 : sp :: r#Transform , r#rectangle_47 : sp :: r#BasicBorderRectangle , r#_Opacity_48 : sp :: r#Opacity , r#rectangle_49 : sp :: r#BasicBorderRectangle , r#image_50 : sp :: r#ClippedImage , r#ta_51 : sp :: r#TouchArea , r#_Opacity_52 : sp :: r#Opacity , r#_Transform_53 : sp :: r#Transform , r#rectangle_54 : sp :: r#BasicBorderRectangle , r#_Opacity_55 : sp :: r#Opacity , r#rectangle_56 : sp :: r#BasicBorderRectangle , r#image_57 : sp :: r#ClippedImage , r#ta_58 : sp :: r#TouchArea , r#_Opacity_60 : sp :: r#Opacity , r#text_61 : sp :: r#SimpleText , r#control_bar_62 : sp :: r#Empty , r#_Opacity_63 : sp :: r#Opacity , r#_shadow_64 : sp :: r#BoxShadow , r#rectangle_65 : sp :: r#BasicBorderRectangle , r#_Opacity_66 : sp :: r#Opacity , r#rectangle_67 : sp :: r#BasicBorderRectangle , r#_Opacity_69 : sp :: r#Opacity , r#_Transform_70 : sp :: r#Transform , r#rectangle_71 : sp :: r#BasicBorderRectangle , r#_Opacity_72 : sp :: r#Opacity , r#rectangle_73 : sp :: r#BasicBorderRectangle , r#image_74 : sp :: r#ClippedImage , r#ta_75 : sp :: r#TouchArea , r#_Opacity_76 : sp :: r#Opacity , r#_Transform_77 : sp :: r#Transform , r#rectangle_78 : sp :: r#BasicBorderRectangle , r#_Opacity_79 : sp :: r#Opacity , r#rectangle_80 : sp :: r#BasicBorderRectangle , r#image_81 : sp :: r#ClippedImage , r#ta_82 : sp :: r#TouchArea , r#_Opacity_83 : sp :: r#Opacity , r#_Transform_84 : sp :: r#Transform , r#rectangle_85 : sp :: r#BasicBorderRectangle , r#_Opacity_86 : sp :: r#Opacity , r#rectangle_87 : sp :: r#BasicBorderRectangle , r#image_88 : sp :: r#ClippedImage , r#ta_89 : sp :: r#TouchArea , r#_Transform_90 : sp :: r#Transform , r#rectangle_91 : sp :: r#BasicBorderRectangle , r#_Opacity_92 : sp :: r#Opacity , r#rectangle_93 : sp :: r#BasicBorderRectangle , r#_Opacity_94 : sp :: r#Opacity , r#image_95 : sp :: r#ClippedImage , r#_Opacity_96 : sp :: r#Opacity , r#image_97 : sp :: r#ClippedImage , r#ta_98 : sp :: r#TouchArea , r#_Opacity_99 : sp :: r#Opacity , r#_Transform_100 : sp :: r#Transform , r#rectangle_101 : sp :: r#BasicBorderRectangle , r#_Opacity_102 : sp :: r#Opacity , r#rectangle_103 : sp :: r#BasicBorderRectangle , r#image_104 : sp :: r#ClippedImage , r#ta_105 : sp :: r#TouchArea , r#_Opacity_106 : sp :: r#Opacity , r#_Transform_107 : sp :: r#Transform , r#rectangle_108 : sp :: r#BasicBorderRectangle , r#_Opacity_109 : sp :: r#Opacity , r#rectangle_110 : sp :: r#BasicBorderRectangle , r#image_111 : sp :: r#ClippedImage , r#ta_112 : sp :: r#TouchArea , r#_Opacity_113 : sp :: r#Opacity , r#_Transform_114 : sp :: r#Transform , r#rectangle_115 : sp :: r#BasicBorderRectangle , r#_Opacity_116 : sp :: r#Opacity , r#rectangle_117 : sp :: r#BasicBorderRectangle , r#image_118 : sp :: r#ClippedImage , r#ta_119 : sp :: r#TouchArea , r#_Opacity_120 : sp :: r#Opacity , r#_Transform_121 : sp :: r#Transform , r#rectangle_122 : sp :: r#BasicBorderRectangle , r#_Opacity_123 : sp :: r#Opacity , r#rectangle_124 : sp :: r#BasicBorderRectangle , r#image_125 : sp :: r#ClippedImage , r#ta_126 : sp :: r#TouchArea , r#_Opacity_127 : sp :: r#Opacity , r#_visibility_128 : sp :: r#Clip , r#rectangle_129 : sp :: r#Empty , r#_Opacity_130 : sp :: r#Opacity , r#_shadow_131 : sp :: r#BoxShadow , r#rectangle_132 : sp :: r#BasicBorderRectangle , r#_Opacity_133 : sp :: r#Opacity , r#rectangle_134 : sp :: r#BasicBorderRectangle , r#text_137 : sp :: r#SimpleText , r#_visibility_138 : sp :: r#Clip , r#rectangle_139 : sp :: r#Rectangle , r#toucharea_140 : sp :: r#TouchArea , r#rectangle_141 : sp :: r#BasicBorderRectangle , r#text_142 : sp :: r#SimpleText , r#text_143 : sp :: r#SimpleText , r#toucharea_144 : sp :: r#TouchArea , r#_Opacity_145 : sp :: r#Opacity , r#_Transform_146 : sp :: r#Transform , r#rectangle_147 : sp :: r#BasicBorderRectangle , r#_Opacity_148 : sp :: r#Opacity , r#rectangle_149 : sp :: r#BasicBorderRectangle , r#image_150 : sp :: r#ClippedImage , r#ta_151 : sp :: r#TouchArea , r#flickable_152 : sp :: r#Flickable , r#_viewport_153 : sp :: r#Empty , r#_visibility_157 : sp :: r#Clip , r#rectangle_158 : sp :: r#Rectangle , r#toucharea_159 : sp :: r#TouchArea , r#_shadow_160 : sp :: r#BoxShadow , r#rectangle_161 : sp :: r#BasicBorderRectangle , r#text_163 : sp :: r#SimpleText , r#text_164 : sp :: r#SimpleText , r#text_165 : sp :: r#ComplexText , r#text_166 : sp :: r#SimpleText , r#text_167 : sp :: r#SimpleText , r#_Opacity_168 : sp :: r#Opacity , r#_Transform_169 : sp :: r#Transform , r#rectangle_170 : sp :: r#BasicBorderRectangle , r#_Opacity_171 : sp :: r#Opacity , r#rectangle_172 : sp :: r#BasicBorderRectangle , r#image_173 : sp :: r#ClippedImage , r#ta_174 : sp :: r#TouchArea , r#waveformarea_59 : InnerWaveformArea_root_1 , r#volumebar_136 : InnerVolumeBar_root_13 , r#root_21__Transform_100_transform_scale : sp :: Property < f32 > , r#root_21__Transform_107_transform_scale : sp :: Property < f32 > , r#root_21__Transform_114_transform_scale : sp :: Property < f32 > , r#root_21__Transform_121_transform_scale : sp :: Property < f32 > , r#root_21__Transform_146_transform_scale : sp :: Property < f32 > , r#root_21__Transform_169_transform_scale : sp :: Property < f32 > , r#root_21__Transform_32_transform_scale : sp :: Property < f32 > , r#root_21__Transform_39_transform_scale : sp :: Property < f32 > , r#root_21__Transform_46_transform_scale : sp :: Property < f32 > , r#root_21__Transform_53_transform_scale : sp :: Property < f32 > , r#root_21__Transform_70_transform_scale : sp :: Property < f32 > , r#root_21__Transform_77_transform_scale : sp :: Property < f32 > , r#root_21__Transform_84_transform_scale : sp :: Property < f32 > , r#root_21__Transform_90_transform_scale : sp :: Property < f32 > , r#root_21_empty_135_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_21_empty_135_layout_cache_ortho : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_21_empty_135_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_21_empty_135_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_21_empty_154_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_21_empty_154_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_21_empty_162_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_21_empty_162_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_21_empty_162_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_21_empty_162_padding_bottom : sp :: Property < sp :: LogicalLength > , r#root_21_empty_162_padding_top : sp :: Property < sp :: LogicalLength > , r#root_21_empty_162_spacing : sp :: Property < sp :: LogicalLength > , r#root_21_empty_68_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_21_empty_68_layout_cache_ortho : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_21_empty_68_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_21_empty_68_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_21_image_29_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_21_image_29_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_23_height : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_23_min_height : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_23_min_width : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_23_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_23_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_23_width : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_24_height : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_24_min_height : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_24_min_width : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_24_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_24_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_24_width : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_25_height : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_25_min_height : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_25_min_width : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_25_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_25_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_21_keybinding_25_width : sp :: Property < sp :: LogicalLength > , r#root_21_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_21_progress_frac : sp :: Property < f32 > , r#root_21_rectangle_101_height : sp :: Property < sp :: LogicalLength > , r#root_21_rectangle_108_height : sp :: Property < sp :: LogicalLength > , r#root_21_rectangle_115_height : sp :: Property < sp :: LogicalLength > , r#root_21_rectangle_122_height : sp :: Property < sp :: LogicalLength > , r#root_21_rectangle_129_x : sp :: Property < sp :: LogicalLength > , r#root_21_rectangle_141_y : sp :: Property < sp :: LogicalLength > , r#root_21_rectangle_71_height : sp :: Property < sp :: LogicalLength > , r#root_21_rectangle_78_height : sp :: Property < sp :: LogicalLength > , r#root_21_rectangle_85_height : sp :: Property < sp :: LogicalLength > , r#root_21_rectangle_91_height : sp :: Property < sp :: LogicalLength > , r#root_21_rectangle_101_clicked : sp :: Callback < () , () > , r#root_21_rectangle_108_clicked : sp :: Callback < () , () > , r#root_21_rectangle_115_clicked : sp :: Callback < () , () > , r#root_21_rectangle_122_clicked : sp :: Callback < () , () > , r#root_21_rectangle_147_clicked : sp :: Callback < () , () > , r#root_21_rectangle_170_clicked : sp :: Callback < () , () > , r#root_21_rectangle_33_clicked : sp :: Callback < () , () > , r#root_21_rectangle_40_clicked : sp :: Callback < () , () > , r#root_21_rectangle_47_clicked : sp :: Callback < () , () > , r#root_21_rectangle_54_clicked : sp :: Callback < () , () > , r#root_21_rectangle_71_clicked : sp :: Callback < () , () > , r#root_21_rectangle_78_clicked : sp :: Callback < () , () > , r#root_21_rectangle_85_clicked : sp :: Callback < () , () > , r#root_21_rectangle_91_clicked : sp :: Callback < () , () > , repeater0 : sp :: Repeater < InnerComponent_playlistrow_155 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerMainWindow >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerMainWindow {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerUIState :: FIELD_OFFSETS . r#playlist () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) as _ }
                 }
            ) ;
             InnerWaveformArea_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 14u32 - 1 , tree_index_of_first_child + 45u32 - 1) ;
             InnerVolumeBar_root_13 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 122u32 - 1 , tree_index_of_first_child + 127u32 - 1) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_100_transform_scale ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_105 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get () {
                         (0.92f64) as _ }
                     else {
                         if (InnerMainWindow :: FIELD_OFFSETS . r#ta_105 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                             (1.05f64) as _ }
                         else {
                             1f64 }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_107_transform_scale ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_112 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get () {
                         (0.92f64) as _ }
                     else {
                         if (InnerMainWindow :: FIELD_OFFSETS . r#ta_112 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                             (1.05f64) as _ }
                         else {
                             1f64 }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_114_transform_scale ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_119 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get () {
                         (0.92f64) as _ }
                     else {
                         if (InnerMainWindow :: FIELD_OFFSETS . r#ta_119 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                             (1.05f64) as _ }
                         else {
                             1f64 }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_121_transform_scale ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_126 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get () {
                         (0.92f64) as _ }
                     else {
                         if (InnerMainWindow :: FIELD_OFFSETS . r#ta_126 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                             (1.05f64) as _ }
                         else {
                             1f64 }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_146_transform_scale ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_151 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get () {
                         (0.92f64) as _ }
                     else {
                         if (InnerMainWindow :: FIELD_OFFSETS . r#ta_151 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                             (1.05f64) as _ }
                         else {
                             1f64 }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_169_transform_scale ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_174 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get () {
                         (0.92f64) as _ }
                     else {
                         if (InnerMainWindow :: FIELD_OFFSETS . r#ta_174 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                             (1.05f64) as _ }
                         else {
                             1f64 }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_32_transform_scale ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_37 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get () {
                         (0.92f64) as _ }
                     else {
                         if (InnerMainWindow :: FIELD_OFFSETS . r#ta_37 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                             (1.05f64) as _ }
                         else {
                             1f64 }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_39_transform_scale ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_44 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get () {
                         (0.92f64) as _ }
                     else {
                         if (InnerMainWindow :: FIELD_OFFSETS . r#ta_44 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                             (1.05f64) as _ }
                         else {
                             1f64 }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_46_transform_scale ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_51 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get () {
                         (0.92f64) as _ }
                     else {
                         if (InnerMainWindow :: FIELD_OFFSETS . r#ta_51 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                             (1.05f64) as _ }
                         else {
                             1f64 }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_53_transform_scale ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_58 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get () {
                         (0.92f64) as _ }
                     else {
                         if (InnerMainWindow :: FIELD_OFFSETS . r#ta_58 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                             (1.05f64) as _ }
                         else {
                             1f64 }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_70_transform_scale ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_75 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get () {
                         (0.92f64) as _ }
                     else {
                         if (InnerMainWindow :: FIELD_OFFSETS . r#ta_75 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                             (1.05f64) as _ }
                         else {
                             1f64 }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_77_transform_scale ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_82 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get () {
                         (0.92f64) as _ }
                     else {
                         if (InnerMainWindow :: FIELD_OFFSETS . r#ta_82 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                             (1.05f64) as _ }
                         else {
                             1f64 }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_84_transform_scale ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_89 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get () {
                         (0.92f64) as _ }
                     else {
                         if (InnerMainWindow :: FIELD_OFFSETS . r#ta_89 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                             (1.05f64) as _ }
                         else {
                             1f64 }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_90_transform_scale ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_98 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get () {
                         (0.92f64) as _ }
                     else {
                         if (InnerMainWindow :: FIELD_OFFSETS . r#ta_98 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                             (1.05f64) as _ }
                         else {
                             1f64 }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("HarmonyOS Sans SC")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_135_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : (sp :: r#LayoutAlignment :: r#Center) . clone () as _ , r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#volumebar_136 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#volumebar_136 . tree_index . get ()))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (92f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (92f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_137 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 123u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (36f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (36f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (12f64) . clone () as _ ;
                             the_struct . r#end = (12f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : (160f64) . clone () as _ , r#spacing : (6f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_135_layout_cache_ortho ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout_ortho (& sp :: BoxLayoutOrthoData {
                         r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#volumebar_136 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#volumebar_136 . tree_index . get ()))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (16f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (16f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_137 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 123u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#cross_axis_alignment : (sp :: r#CrossAxisAlignment :: r#Center) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : (40f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_135_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#volumebar_136 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#volumebar_136 . tree_index . get ()))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (92f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (92f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_137 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 123u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (36f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (36f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , 6f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (12f64) . clone () as _ ;
                         the_struct . r#end = (12f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_135_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#volumebar_136 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#volumebar_136 . tree_index . get ()))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (16f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (16f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_137 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 123u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (0f64) . clone () as _ ;
                         the_struct . r#end = (0f64) . clone () as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_154_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerMainWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                         r#repeated_indices [0usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [0usize + 1] = _self . repeater0 . len () as u32 ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (r#cells) . clone () as _ , r#padding : ({
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = (0f64) . clone () as _ ;
                                 the_struct . r#end = (0f64) . clone () as _ ;
                                 the_struct }
                            ) . clone () as _ , r#size : ((InnerMainWindow :: FIELD_OFFSETS . r#flickable_152 () + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ , r#spacing : (0f64) . clone () as _ , }
                         as _ , r#repeated_indices as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_154_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerMainWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_163 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 148u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_164 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 149u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_165 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 150u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_166 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 151u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_167 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 152u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (14f64) . clone () as _ ;
                             the_struct . r#end = (12f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : (136f64) . clone () as _ , r#spacing : (5f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_163 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 148u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_164 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 149u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_165 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 150u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_166 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 151u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_167 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 152u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (18f64) . clone () as _ ;
                         the_struct . r#end = (18f64) . clone () as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_163 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 148u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_164 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 149u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_165 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 150u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_166 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 151u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_167 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 152u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , 5f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (14f64) . clone () as _ ;
                         the_struct . r#end = (12f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_padding_bottom ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_padding_top ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_spacing ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : (sp :: r#LayoutAlignment :: r#Center) . clone () as _ , r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_71 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 69u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (28f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (28f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_78 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 75u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (28f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (28f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_85 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 81u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (28f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (28f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_91 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 86u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (34f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (34f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_101 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 95u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (28f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (28f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_108 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 101u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (28f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (28f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_115 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 107u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (28f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (28f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_122 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 113u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (28f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (28f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (10f64) . clone () as _ ;
                             the_struct . r#end = (10f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : (300f64) . clone () as _ , r#spacing : (7f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache_ortho ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout_ortho (& sp :: BoxLayoutOrthoData {
                         r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_71 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 69u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (28f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (28f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_78 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 75u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (28f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (28f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_85 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 81u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (28f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (28f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_91 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 86u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (34f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (34f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_101 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 95u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (28f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (28f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_108 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 101u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (28f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (28f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_115 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 107u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (28f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (28f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_122 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 113u32 - 1))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (28f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (28f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#cross_axis_alignment : (sp :: r#CrossAxisAlignment :: r#Center) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : (40f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_71 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 69u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (28f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (28f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_78 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 75u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (28f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (28f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_85 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 81u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (28f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (28f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_91 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 86u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (34f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (34f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_101 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 95u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (28f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (28f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_108 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 101u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (28f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (28f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_115 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 107u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (28f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (28f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_122 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 113u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (28f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (28f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , 7f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (10f64) . clone () as _ ;
                         the_struct . r#end = (10f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_71 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 69u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (28f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (28f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_78 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 75u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (28f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (28f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_85 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 81u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (28f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (28f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_91 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 86u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (34f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (34f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_101 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 95u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (28f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (28f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_108 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 101u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (28f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (28f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_115 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 107u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (28f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (28f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_122 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 113u32 - 1))) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (28f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (28f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (0f64) . clone () as _ ;
                         the_struct . r#end = (0f64) . clone () as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (170f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_image_29_preferred_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#image_29 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 8u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_image_29_preferred_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#image_29 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 8u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_23_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_23_preferred_height ()) . apply_pin (_self) . get () . get () . max ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_23_min_height ()) . apply_pin (_self) . get () . get ()) as sp :: Coord)) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_23_min_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#keybinding_23 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_23_min_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#keybinding_23 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_23_preferred_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#keybinding_23 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_23_preferred_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#keybinding_23 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_23_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_23_preferred_width ()) . apply_pin (_self) . get () . get () . max ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_23_min_width ()) . apply_pin (_self) . get () . get ()) as sp :: Coord)) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_24_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_24_preferred_height ()) . apply_pin (_self) . get () . get () . max ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_24_min_height ()) . apply_pin (_self) . get () . get ()) as sp :: Coord)) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_24_min_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#keybinding_24 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 5u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_24_min_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#keybinding_24 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 5u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_24_preferred_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#keybinding_24 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 5u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_24_preferred_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#keybinding_24 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 5u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_24_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_24_preferred_width ()) . apply_pin (_self) . get () . get () . max ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_24_min_width ()) . apply_pin (_self) . get () . get ()) as sp :: Coord)) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_25_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_25_preferred_height ()) . apply_pin (_self) . get () . get () . max ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_25_min_height ()) . apply_pin (_self) . get () . get ()) as sp :: Coord)) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_25_min_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#keybinding_25 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 6u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_25_min_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#keybinding_25 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 6u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_25_preferred_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#keybinding_25 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 6u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_25_preferred_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#keybinding_25 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 6u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_25_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_25_preferred_width ()) . apply_pin (_self) . get () . get () . max ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_25_min_width ()) . apply_pin (_self) . get () . get ()) as sp :: Coord)) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#root_21 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone ())) + ((((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = (0f64) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                    ) . clone ())) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_image_29_preferred_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#stretch = (0f64) . clone () as _ ;
                         the_struct }
                    ) . clone ())))) . clone ())))) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_progress_frac ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if {
                         * & InnerUIState :: FIELD_OFFSETS . r#dragging () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () {
                         ({
                             * & InnerUIState :: FIELD_OFFSETS . r#drag_fraction () }
                         . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) as _ }
                     else {
                         {
                             let r#tmp_UIState_duration = ({
                                 * & InnerUIState :: FIELD_OFFSETS . r#duration () }
                             . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) . clone () ;
                             if (((r#tmp_UIState_duration) . clone ()) as f64) > (((0f64) . clone ()) as f64) {
                                 (((((({
                                     * & InnerUIState :: FIELD_OFFSETS . r#position () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) . clone ()) as f64) / (((r#tmp_UIState_duration) . clone ()) as f64)) as f32) . min (1f64 as f32)) as _ }
                             else {
                                 0f64 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_101_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_next () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#next () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_101_height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (28f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_108_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_seek_relative () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#seek_relative () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ((5f64) . clone () as _ ,)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_108_height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (28f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_115_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_toggle_volume_popup () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#toggle_volume_popup () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_115_height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (28f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_122_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_toggle_playlist () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#toggle_playlist () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_122_height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (28f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_129_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if {
                         * & InnerUIState :: FIELD_OFFSETS . r#volume_popup_open () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () {
                         (520f64) as _ }
                     else {
                         476f64 }
                     as sp :: Coord)) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_141_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if {
                         * & InnerUIState :: FIELD_OFFSETS . r#playlist_open () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () {
                         (14f64) as _ }
                     else {
                         170f64 }
                     as sp :: Coord)) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (220f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_147_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_toggle_playlist () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#toggle_playlist () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_170_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 * & InnerUIState :: FIELD_OFFSETS . r#about_open () }
                             . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . set (false as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_33_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 * & InnerUIState :: FIELD_OFFSETS . r#about_open () }
                             . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . set (true as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_40_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_minimize_window () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#minimize_window () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_47_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_toggle_pin () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#toggle_pin () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_54_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_close_window () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#close_window () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_71_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_cycle_mode () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#cycle_mode () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_71_height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (28f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_78_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_seek_relative () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#seek_relative () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& (((- 5f64)) . clone () as _ ,)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_78_height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (28f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_85_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_previous () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#previous () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_85_height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (28f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_91_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_toggle_play () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#toggle_play () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_91_height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (34f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#title ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("zzhMusicPlayer")) as sp :: SharedString }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (720f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#shortcuts_22 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#shortcuts_22 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#shortcuts_22 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#keybinding_23 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#activated ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_toggle_play () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#toggle_play () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#keybinding_23 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#keybinding_23 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#keys ()) . apply_pin (_self) . set ({
                 (sp :: make_keys (" " . into () , {
                     let mut modifiers = sp :: KeyboardModifiers :: default () ;
                     modifiers . alt = false ;
                     modifiers . control = false ;
                     modifiers . shift = false ;
                     modifiers . meta = false ;
                     modifiers }
                 , false , false)) as sp :: Keys }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#keybinding_24 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#activated ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_seek_relative () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#seek_relative () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& (((- 5f64)) . clone () as _ ,)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#keybinding_24 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#keybinding_24 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#keys ()) . apply_pin (_self) . set ({
                 (sp :: make_keys ("\u{f702}" . into () , {
                     let mut modifiers = sp :: KeyboardModifiers :: default () ;
                     modifiers . alt = false ;
                     modifiers . control = false ;
                     modifiers . shift = false ;
                     modifiers . meta = false ;
                     modifiers }
                 , false , false)) as sp :: Keys }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#keybinding_25 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#activated ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_seek_relative () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#seek_relative () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ((5f64) . clone () as _ ,)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#keybinding_25 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#keybinding_25 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#keys ()) . apply_pin (_self) . set ({
                 (sp :: make_keys ("\u{f703}" . into () , {
                     let mut modifiers = sp :: KeyboardModifiers :: default () ;
                     modifiers . alt = false ;
                     modifiers . control = false ;
                     modifiers . shift = false ;
                     modifiers . meta = false ;
                     modifiers }
                 , false , false)) as sp :: Keys }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_26 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#toucharea_26 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((((args . 0 . clone ()) . r#kind) . clone ())) == (((sp :: r#PointerEventKind :: r#Down) . clone ())) {
                                 ({
                                     {
                                         {
                                             * & InnerUIState :: FIELD_OFFSETS . callback_tracker_window_drag_down () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                         {
                                             * & InnerUIState :: FIELD_OFFSETS . r#window_drag_down () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& (((((((InnerMainWindow :: FIELD_OFFSETS . r#toucharea_26 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) / (((1f64) . clone ()) as f64))) . clone () as _ , ((((((InnerMainWindow :: FIELD_OFFSETS . r#toucharea_26 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) / (((1f64) . clone ()) as f64))) . clone () as _ ,)) ;
                                         }
                                     }
                                ) ;
                                 }
                             else {
                                 if ((((args . 0 . clone ()) . r#kind) . clone ())) == (((sp :: r#PointerEventKind :: r#Move) . clone ())) {
                                     ({
                                         {
                                             {
                                                 * & InnerUIState :: FIELD_OFFSETS . callback_tracker_window_drag_move () }
                                             . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                             {
                                                 * & InnerUIState :: FIELD_OFFSETS . r#window_drag_move () }
                                             . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& (((((((InnerMainWindow :: FIELD_OFFSETS . r#toucharea_26 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) / (((1f64) . clone ()) as f64))) . clone () as _ , ((((((InnerMainWindow :: FIELD_OFFSETS . r#toucharea_26 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) / (((1f64) . clone ()) as f64))) . clone () as _ ,)) ;
                                             }
                                         }
                                    ) ;
                                     }
                                 else {
                                     if ((((args . 0 . clone ()) . r#kind) . clone ())) == (((sp :: r#PointerEventKind :: r#Up) . clone ())) {
                                         ({
                                             {
                                                 {
                                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_window_drag_up () }
                                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                                 {
                                                     * & InnerUIState :: FIELD_OFFSETS . r#window_drag_up () }
                                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ()) ;
                                                 }
                                             }
                                        ) ;
                                         }
                                     else {
                                         {
                                             }
                                         }
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4279507236f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_clip_28 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_clip_28 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_clip_28 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_clip_28 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_clip_28 () + sp :: r#Clip :: FIELD_OFFSETS . r#clip ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((((1f64) . clone ()) as f64) * (((170f64) . clone ()) as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Fill) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerUIState :: FIELD_OFFSETS . r#bg_image () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((((1f64) . clone ()) as f64) * (((720f64) . clone ()) as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4294112504f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set ({
                 (((600f64) as i32)) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (22f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Left) as sp :: r#TextHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#overflow ()) . apply_pin (_self) . set ({
                 (sp :: r#TextOverflow :: r#Elide) as sp :: r#TextOverflow }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_UIState_track_title = ({
                             * & InnerUIState :: FIELD_OFFSETS . r#track_title () }
                         . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) . clone () ;
                         if (((r#tmp_UIState_track_title) . clone ())) == (((sp :: SharedString :: from ("")) . clone ())) {
                             (sp :: SharedString :: from ("")) as _ }
                         else {
                             {
                                 let r#tmp_UIState_track_artist = ({
                                     * & InnerUIState :: FIELD_OFFSETS . r#track_artist () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) . clone () ;
                                 if (((r#tmp_UIState_track_artist) . clone ())) == (((sp :: SharedString :: from ("")) . clone ())) {
                                     (r#tmp_UIState_track_title) as _ }
                                 else {
                                     (((((((r#tmp_UIState_track_title) . clone ())) + (((sp :: SharedString :: from (" — ")) . clone ()) . as_str ()))) . clone ())) + (((r#tmp_UIState_track_artist) . clone ()) . as_str ()) }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (510f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_31 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_37 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.95f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_32 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_origin ()) . apply_pin (_self) . set ({
                 ({
                     let mut the_struct = slint :: LogicalPosition :: default () ;
                     the_struct . r#x = (13f64) . clone () as _ ;
                     the_struct . r#y = (13f64) . clone () as _ ;
                     the_struct }
                ) as slint :: LogicalPosition }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_32 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_32 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_32_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_32 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_32_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_33 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((2149985078f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_33 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_34 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_37 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.2f64) as _ }
                     else {
                         0.12f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_35 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_35 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14.560000000000002f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageHorizontalAlignment :: r#Center) as sp :: r#ImageHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_6 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_6 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_6 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14.560000000000002f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#ta_37 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_33_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_37 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_37 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_38 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_44 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.95f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_39 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_origin ()) . apply_pin (_self) . set ({
                 ({
                     let mut the_struct = slint :: LogicalPosition :: default () ;
                     the_struct . r#x = (13f64) . clone () as _ ;
                     the_struct . r#y = (13f64) . clone () as _ ;
                     the_struct }
                ) as slint :: LogicalPosition }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_39 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_39 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_39_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_39 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_39_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_40 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((2149985078f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_40 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_41 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_44 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.2f64) as _ }
                     else {
                         0.12f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_42 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_42 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14.560000000000002f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageHorizontalAlignment :: r#Center) as sp :: r#ImageHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_7 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_7 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_7 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14.560000000000002f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#ta_44 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_40_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_44 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_44 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_45 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_51 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.95f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_46 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_origin ()) . apply_pin (_self) . set ({
                 ({
                     let mut the_struct = slint :: LogicalPosition :: default () ;
                     the_struct . r#x = (13f64) . clone () as _ ;
                     the_struct . r#y = (13f64) . clone () as _ ;
                     the_struct }
                ) as slint :: LogicalPosition }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_46 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_46 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_46_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_46 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_46_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_47 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerUIState :: FIELD_OFFSETS . r#always_on_top () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((3426566480f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((2149985078f64) as u32) }
                    )) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_47 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_48 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_51 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.2f64) as _ }
                     else {
                         0.12f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_49 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_49 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14.560000000000002f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageHorizontalAlignment :: r#Center) as sp :: r#ImageHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_8 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_8 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_8 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14.560000000000002f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#ta_51 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_47_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_51 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_51 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_52 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_58 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.95f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_53 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_origin ()) . apply_pin (_self) . set ({
                 ({
                     let mut the_struct = slint :: LogicalPosition :: default () ;
                     the_struct . r#x = (13f64) . clone () as _ ;
                     the_struct . r#y = (13f64) . clone () as _ ;
                     the_struct }
                ) as slint :: LogicalPosition }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_53 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_53 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_53_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_53 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_53_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_54 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((2149985078f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_54 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_55 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_58 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.2f64) as _ }
                     else {
                         0.12f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_56 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_56 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14.560000000000002f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageHorizontalAlignment :: r#Center) as sp :: r#ImageHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_5 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_5 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_5 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14.560000000000002f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#ta_58 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_54_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_58 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_58 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (93.50000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_progress_frac ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_progress_frac ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (720f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_x ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_y ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_60 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if {
                         * & InnerUIState :: FIELD_OFFSETS . r#mode_showing () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (120f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: Linear) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#text_61 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4293454576f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_61 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (11f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_61 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (18f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_61 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_61 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerUIState :: FIELD_OFFSETS . r#mode_text () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#text_61 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_61 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (130f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_63 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if {
                         * & InnerUIState :: FIELD_OFFSETS . r#toolbar_hovered () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () {
                         (0.88f64) as _ }
                     else {
                         0.3f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded ((1493172224f64) as u32)) as sp :: Color }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (3f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_65 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4279441443f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_65 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_66 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if {
                         * & InnerUIState :: FIELD_OFFSETS . r#toolbar_hovered () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () {
                         (0.18f64) as _ }
                     else {
                         0.08f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_67 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_67 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_69 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_75 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.95f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_70 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_origin ()) . apply_pin (_self) . set ({
                 ({
                     let mut the_struct = slint :: LogicalPosition :: default () ;
                     the_struct . r#x = (14f64) . clone () as _ ;
                     the_struct . r#y = (14f64) . clone () as _ ;
                     the_struct }
                ) as slint :: LogicalPosition }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_70 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_70 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_70_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_70 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_70_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_71 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((2149985078f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_71 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_72 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_75 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.2f64) as _ }
                     else {
                         0.12f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_73 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_73 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15.680000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageHorizontalAlignment :: r#Center) as sp :: r#ImageHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_9 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_9 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_9 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15.680000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#ta_75 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_71_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_75 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_75 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_76 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_82 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.95f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_77 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_origin ()) . apply_pin (_self) . set ({
                 ({
                     let mut the_struct = slint :: LogicalPosition :: default () ;
                     the_struct . r#x = (14f64) . clone () as _ ;
                     the_struct . r#y = (14f64) . clone () as _ ;
                     the_struct }
                ) as slint :: LogicalPosition }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_77 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_77 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_77_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_77 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_77_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_78 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((2149985078f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_78 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_79 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_82 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.2f64) as _ }
                     else {
                         0.12f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_80 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_80 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15.680000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageHorizontalAlignment :: r#Center) as sp :: r#ImageHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_10 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_10 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_10 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15.680000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#ta_82 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_78_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_82 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_82 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_83 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_89 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.95f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_84 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_origin ()) . apply_pin (_self) . set ({
                 ({
                     let mut the_struct = slint :: LogicalPosition :: default () ;
                     the_struct . r#x = (14f64) . clone () as _ ;
                     the_struct . r#y = (14f64) . clone () as _ ;
                     the_struct }
                ) as slint :: LogicalPosition }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_84 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_84 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_84_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_84 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_84_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_85 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((2149985078f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_85 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_86 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_89 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.2f64) as _ }
                     else {
                         0.12f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_87 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_87 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15.680000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageHorizontalAlignment :: r#Center) as sp :: r#ImageHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_11 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_11 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_11 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15.680000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#ta_89 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_85_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_89 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_89 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_90 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_origin ()) . apply_pin (_self) . set ({
                 ({
                     let mut the_struct = slint :: LogicalPosition :: default () ;
                     the_struct . r#x = (17f64) . clone () as _ ;
                     the_struct . r#y = (17f64) . clone () as _ ;
                     the_struct }
                ) as slint :: LogicalPosition }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_90 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_90 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_90_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_90 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_90_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_91 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((3006018112f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_91 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (17f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_92 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_98 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.2f64) as _ }
                     else {
                         0.12f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_93 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_93 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (17f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_94 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if {
                         * & InnerUIState :: FIELD_OFFSETS . r#playing () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: Linear) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (19.040000000000003f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageHorizontalAlignment :: r#Center) as sp :: r#ImageHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_12 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_12 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_12 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (19.040000000000003f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_96 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if {
                         * & InnerUIState :: FIELD_OFFSETS . r#playing () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () {
                         (0f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: Linear) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (19.040000000000003f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageHorizontalAlignment :: r#Center) as sp :: r#ImageHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_13 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_13 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_13 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (19.040000000000003f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#ta_98 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_91_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_98 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_98 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_99 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_105 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.95f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_100 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_origin ()) . apply_pin (_self) . set ({
                 ({
                     let mut the_struct = slint :: LogicalPosition :: default () ;
                     the_struct . r#x = (14f64) . clone () as _ ;
                     the_struct . r#y = (14f64) . clone () as _ ;
                     the_struct }
                ) as slint :: LogicalPosition }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_100 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_100 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_100_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_100 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_100_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_101 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((2149985078f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_101 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_102 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_105 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.2f64) as _ }
                     else {
                         0.12f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_103 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_103 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15.680000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageHorizontalAlignment :: r#Center) as sp :: r#ImageHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15.680000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#ta_105 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_101_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_105 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_105 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_106 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_112 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.95f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_107 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_origin ()) . apply_pin (_self) . set ({
                 ({
                     let mut the_struct = slint :: LogicalPosition :: default () ;
                     the_struct . r#x = (14f64) . clone () as _ ;
                     the_struct . r#y = (14f64) . clone () as _ ;
                     the_struct }
                ) as slint :: LogicalPosition }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_107 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_107 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_107_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_107 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_107_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_108 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((2149985078f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_108 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_109 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_112 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.2f64) as _ }
                     else {
                         0.12f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_110 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_110 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15.680000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageHorizontalAlignment :: r#Center) as sp :: r#ImageHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15.680000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#ta_112 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_108_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_112 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_112 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_113 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_119 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.95f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_114 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_origin ()) . apply_pin (_self) . set ({
                 ({
                     let mut the_struct = slint :: LogicalPosition :: default () ;
                     the_struct . r#x = (14f64) . clone () as _ ;
                     the_struct . r#y = (14f64) . clone () as _ ;
                     the_struct }
                ) as slint :: LogicalPosition }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_114 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_114 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_114_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_114 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_114_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_115 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerUIState :: FIELD_OFFSETS . r#volume_popup_open () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((3426566480f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((2149985078f64) as u32) }
                    )) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_115 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_116 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_119 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.2f64) as _ }
                     else {
                         0.12f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_117 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_117 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15.680000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageHorizontalAlignment :: r#Center) as sp :: r#ImageHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15.680000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#ta_119 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_115_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_119 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_119 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_120 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_126 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.95f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_121 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_origin ()) . apply_pin (_self) . set ({
                 ({
                     let mut the_struct = slint :: LogicalPosition :: default () ;
                     the_struct . r#x = (14f64) . clone () as _ ;
                     the_struct . r#y = (14f64) . clone () as _ ;
                     the_struct }
                ) as slint :: LogicalPosition }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_121 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_121 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_121_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_121 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_121_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_122 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerUIState :: FIELD_OFFSETS . r#playlist_open () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((3426566480f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((2149985078f64) as u32) }
                    )) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_122 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_123 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_126 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.2f64) as _ }
                     else {
                         0.12f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_124 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_124 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15.680000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageHorizontalAlignment :: r#Center) as sp :: r#ImageHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_4 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_4 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_4 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15.680000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#ta_126 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_122_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_126 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_126 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_127 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if {
                         * & InnerUIState :: FIELD_OFFSETS . r#volume_popup_open () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (120f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_visibility_128 () + sp :: r#Clip :: FIELD_OFFSETS . r#clip ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! {
                         * & InnerUIState :: FIELD_OFFSETS . r#volume_popup_open () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_130 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) . set ({
                 (0.88f64) as f32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded ((1493172224f64) as u32)) as sp :: Color }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (3f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_132 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4279441443f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_132 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_133 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) . set ({
                 (0.18f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_134 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_134 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_changed ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_set_volume () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#set_volume () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ((args . 0 . clone ()) . clone () as _ ,)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_volume ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerUIState :: FIELD_OFFSETS . r#volume () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (92f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_135_layout_cache ()) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_135_layout_cache_ortho ()) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#text_137 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4293454576f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_137 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (11f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_137 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_135_layout_cache_ortho ()) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#text_137 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Right) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_137 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerUIState :: FIELD_OFFSETS . r#volume_text () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#text_137 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_137 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (36f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_visibility_138 () + sp :: r#Clip :: FIELD_OFFSETS . r#clip ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! {
                         * & InnerUIState :: FIELD_OFFSETS . r#playlist_open () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_139 () + sp :: r#Rectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((1711276032f64) as u32))) as slint :: Brush }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#toucharea_140 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_toggle_playlist () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#toggle_playlist () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_140 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_141 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4061469219f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_141 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_142 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4294112504f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_142 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_142 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set ({
                 (((600f64) as i32)) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_142 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_142 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("播放列表")) as sp :: SharedString }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_142 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_142 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (140f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_143 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4288324530f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_143 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_143 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_143 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_143 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("清空")) as sp :: SharedString }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_143 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_143 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (44f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#toucharea_144 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_clear_playlist () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#clear_playlist () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_144 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_144 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_145 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_151 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.95f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_146 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_origin ()) . apply_pin (_self) . set ({
                 ({
                     let mut the_struct = slint :: LogicalPosition :: default () ;
                     the_struct . r#x = (12f64) . clone () as _ ;
                     the_struct . r#y = (12f64) . clone () as _ ;
                     the_struct }
                ) as slint :: LogicalPosition }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_146 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_146 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_146_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_146 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_146_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_147 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((2149985078f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_147 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_148 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_151 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.2f64) as _ }
                     else {
                         0.12f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_149 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_149 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13.440000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageHorizontalAlignment :: r#Center) as sp :: r#ImageHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_5 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_5 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_5 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13.440000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#ta_151 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_147_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_151 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_151 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#flickable_152 () + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#flickable_152 () + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((match & ({
                         * & InnerUIState :: FIELD_OFFSETS . r#playlist () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) . clone () {
                         x => {
                             x . model_tracker () . track_row_count_changes () ;
                             x . row_count () as i32 }
                         }
                    ) . clone ()) as f64) * (((26f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#flickable_152 () + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((704f64 as sp :: Coord) . max (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_154_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_visibility_157 () + sp :: r#Clip :: FIELD_OFFSETS . r#clip ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! {
                         * & InnerUIState :: FIELD_OFFSETS . r#about_open () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_158 () + sp :: r#Rectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((1711276032f64) as u32))) as slint :: Brush }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#toucharea_159 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 * & InnerUIState :: FIELD_OFFSETS . r#about_open () }
                             . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . set (false as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_159 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded ((2281701376f64) as u32)) as sp :: Color }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_161 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4111932454f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_161 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_163 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4294112504f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_163 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_163 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set ({
                 (((600f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_163 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#text_163 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("关于 zzhMusicPlayer")) as sp :: SharedString }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_163 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (304f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_164 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4291810525f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_164 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_164 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#text_164 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("开发者：zzhzhouzhou")) as sp :: SharedString }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_164 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (304f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4291810525f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) . get () [5usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("简单、轻量、纯粹的音频播放器")) as sp :: SharedString }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (304f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#wrap ()) . apply_pin (_self) . set ({
                 (sp :: r#TextWrap :: r#WordWrap) as sp :: r#TextWrap }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_166 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4291810525f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_166 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_166 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) . get () [7usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#text_166 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("技术：Rust · Slint · rodio · symphonia")) as sp :: SharedString }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_166 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (304f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_167 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4291810525f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_167 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#text_167 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) . get () [9usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#text_167 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("版本：1.0.0")) as sp :: SharedString }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_167 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (304f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_168 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_174 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.95f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (100f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_169 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_origin ()) . apply_pin (_self) . set ({
                 ({
                     let mut the_struct = slint :: LogicalPosition :: default () ;
                     the_struct . r#x = (12f64) . clone () as _ ;
                     the_struct . r#y = (12f64) . clone () as _ ;
                     the_struct }
                ) as slint :: LogicalPosition }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_169 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_169 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_169_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Transform_169 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_scale_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_21__Transform_169_transform_scale ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_170 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((2149985078f64) as u32))) as slint :: Brush }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_170 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_171 () + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (InnerMainWindow :: FIELD_OFFSETS . r#ta_174 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                         (0.2f64) as _ }
                     else {
                         0.12f64 }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32])) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_172 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor ({
                         * & InnerUIState :: FIELD_OFFSETS . r#theme_color () }
                     . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ())) as _ }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_172 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13.440000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageHorizontalAlignment :: r#Center) as sp :: r#ImageHorizontalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageRendering :: r#Smooth) as sp :: r#ImageRendering }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_5 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_5 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#height) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set ({
                 (((((((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_5 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . r#width) . clone ()) as f64) - (((((0f64) as i32)) . clone ()) as f64))) as i32 }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageVerticalAlignment :: r#Center) as sp :: r#ImageVerticalAlignment }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13.440000000000001f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerMainWindow :: FIELD_OFFSETS . r#ta_174 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_170_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_174 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_174 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set ({
                 (sp :: r#MouseCursor :: r#Pointer) as sp :: r#MouseCursor }
            ) ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_padding_bottom ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_padding_top ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_spacing ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_101_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_108_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_115_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_122_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_71_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_78_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_85_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_91_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#resize_border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#root_21 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#title ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#shortcuts_22 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#shortcuts_22 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#shortcuts_22 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#keybinding_23 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#keybinding_23 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#keys ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#keybinding_24 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#keybinding_24 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#keys ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#keybinding_25 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#keybinding_25 () + sp :: r#KeyBinding :: FIELD_OFFSETS . r#keys ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_26 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_26 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_clip_28 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_clip_28 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_clip_28 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_clip_28 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_clip_28 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_family ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_italic ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#letter_spacing ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#overflow ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke_style ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#wrap ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_32 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_33 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_33 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_33 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_33 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_35 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_35 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_35 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_36 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_37 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_37 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_39 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_40 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_40 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_40 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_40 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_42 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_42 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_42 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_43 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_44 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_44 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_46 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_47 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_47 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_47 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_49 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_49 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_49 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_50 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_51 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_51 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_53 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_54 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_54 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_54 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_54 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_56 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_56 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_56 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_57 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_58 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_58 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#Rectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_61 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_61 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_61 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_61 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_61 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_61 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_61 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#inset ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#spread ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_65 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_65 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_65 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_65 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_67 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_67 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_67 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_70 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_71 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_71 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_71 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_71 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_73 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_73 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_73 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_74 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_75 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_75 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_77 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_78 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_78 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_78 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_78 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_80 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_80 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_80 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_81 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_82 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_82 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_84 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_85 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_85 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_85 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_85 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_87 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_87 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_87 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_88 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_89 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_89 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_90 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_91 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_91 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_91 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_91 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_93 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_93 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_93 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_95 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_97 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_98 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_98 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_100 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_101 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_101 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_101 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_101 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_103 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_103 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_103 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_104 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_105 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_105 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_107 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_108 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_108 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_108 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_108 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_110 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_110 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_110 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_111 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_112 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_112 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_114 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_115 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_115 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_115 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_117 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_117 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_117 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_118 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_119 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_119 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_121 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_122 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_122 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_122 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_124 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_124 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_124 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_125 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_126 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_126 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_128 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_128 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_128 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_128 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_128 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#inset ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#spread ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_132 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_132 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_132 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_132 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_134 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_134 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_134 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_137 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_137 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_137 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_137 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_137 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_137 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_138 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_138 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_138 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_138 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_138 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_139 () + sp :: r#Rectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_140 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_140 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_141 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_141 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_141 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_141 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_142 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_142 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_142 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_142 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_142 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_142 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_142 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_142 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_143 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_143 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_143 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_143 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_143 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_143 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_143 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_143 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_144 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_144 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_146 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_147 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_147 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_147 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_147 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_149 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_149 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_149 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_150 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_151 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_151 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#flickable_152 () + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_157 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_157 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_157 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_157 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_157 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_158 () + sp :: r#Rectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_159 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_159 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#inset ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 () + sp :: r#BoxShadow :: FIELD_OFFSETS . r#spread ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_161 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_161 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_161 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_161 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_163 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_163 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_163 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_163 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_163 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_163 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_163 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_164 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_164 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_164 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_164 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_164 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_164 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_164 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_family ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_italic ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#letter_spacing ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#overflow ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke_style ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_165 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#wrap ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_166 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_166 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_166 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_166 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_166 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_166 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_166 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_167 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_167 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_167 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_167 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_167 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_167 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#text_167 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_169 () + sp :: r#Transform :: FIELD_OFFSETS . r#transform_rotation ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_170 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_170 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_170 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_170 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_172 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_172 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_172 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#horizontal_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_height ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_x ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#source_clip_y ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#vertical_tiling ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#image_173 () + sp :: r#ClippedImage :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_174 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set_constant () ;
             (InnerMainWindow :: FIELD_OFFSETS . r#ta_174 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerWaveformArea_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (x)) ,) ;
             InnerVolumeBar_root_13 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () . apply_pin (x)) ,) ;
             _self . r#fn_focus () ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . visit (order , visitor) }
                 1u32 ..= 1u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (_self) . visit_dynamic_children (dyn_index - 1u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             {
                 _changed |= InnerMainWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_playlistrow_155 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                 }
             _changed |= InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (_self) . ensure_instantiated () ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (720f64) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (720f64) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (_self . r#fn_layoutinfo_v_with_constraint (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#preferred as _)) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (170f64) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (170f64) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 ..= 1u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (_self) . subtree_range (dyn_index - 1u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 ..= 1u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (_self) . subtree_component (dyn_index - 1u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((170f64) . clone ()) . clone () as sp :: Coord , ((720f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 => (((170f64) . clone ()) . clone () as sp :: Coord , ((720f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 2u32 => (((170f64) . clone ()) . clone () as sp :: Coord , ((720f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 3u32 => (((170f64) . clone ()) . clone () as sp :: Coord , ((720f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 4u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_23_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_23_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((((((((((720f64) . clone ()) as f64) - ((((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_23_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((((((170f64) . clone ()) as f64) - ((((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_23_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord ,) , 5u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_24_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_24_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((((((((((720f64) . clone ()) as f64) - ((((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_24_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((((((170f64) . clone ()) as f64) - ((((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_24_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord ,) , 6u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_25_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_25_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((((((((((720f64) . clone ()) as f64) - ((((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_25_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((((((170f64) . clone ()) as f64) - ((((InnerMainWindow :: FIELD_OFFSETS . r#root_21_keybinding_25_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord ,) , 7u32 => (((170f64) . clone ()) . clone () as sp :: Coord , ((720f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 8u32 => (((((((1f64) . clone ()) as f64) * (((170f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * (((720f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 9u32 => (((22f64) . clone ()) . clone () as sp :: Coord , ((510f64) . clone ()) . clone () as sp :: Coord , ((16f64) . clone ()) . clone () as sp :: Coord , ((10f64) . clone ()) . clone () as sp :: Coord ,) , 10u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((582f64) . clone ()) . clone () as sp :: Coord , ((7f64) . clone ()) . clone () as sp :: Coord ,) , 11u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((614f64) . clone ()) . clone () as sp :: Coord , ((7f64) . clone ()) . clone () as sp :: Coord ,) , 12u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((646f64) . clone ()) . clone () as sp :: Coord , ((7f64) . clone ()) . clone () as sp :: Coord ,) , 13u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((678f64) . clone ()) . clone () as sp :: Coord , ((7f64) . clone ()) . clone () as sp :: Coord ,) , 14u32 => (((93.50000000000001f64) . clone ()) . clone () as sp :: Coord , ((720f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((40f64) . clone ()) . clone () as sp :: Coord ,) , 15u32 => (((18f64) . clone ()) . clone () as sp :: Coord , ((130f64) . clone ()) . clone () as sp :: Coord , ((295f64) . clone ()) . clone () as sp :: Coord , ((98f64) . clone ()) . clone () as sp :: Coord ,) , 16u32 => (((40f64) . clone ()) . clone () as sp :: Coord , ((300f64) . clone ()) . clone () as sp :: Coord , ((210f64) . clone ()) . clone () as sp :: Coord , ((120f64) . clone ()) . clone () as sp :: Coord ,) , 17u32 => (((40f64) . clone ()) . clone () as sp :: Coord , ((160f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_129_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((120f64) . clone ()) . clone () as sp :: Coord ,) , 18u32 => (((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 19u32 => (((156f64) . clone ()) . clone () as sp :: Coord , ((720f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_141_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 20u32 => (((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 21u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 22u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 23u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 24u32 => (((14.560000000000002f64) . clone ()) . clone () as sp :: Coord , ((14.560000000000002f64) . clone ()) . clone () as sp :: Coord , ((5.719999999999999f64) . clone ()) . clone () as sp :: Coord , ((5.719999999999999f64) . clone ()) . clone () as sp :: Coord ,) , 25u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 26u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 27u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 28u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 29u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 30u32 => (((14.560000000000002f64) . clone ()) . clone () as sp :: Coord , ((14.560000000000002f64) . clone ()) . clone () as sp :: Coord , ((5.719999999999999f64) . clone ()) . clone () as sp :: Coord , ((5.719999999999999f64) . clone ()) . clone () as sp :: Coord ,) , 31u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 32u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 33u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 34u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 35u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 36u32 => (((14.560000000000002f64) . clone ()) . clone () as sp :: Coord , ((14.560000000000002f64) . clone ()) . clone () as sp :: Coord , ((5.719999999999999f64) . clone ()) . clone () as sp :: Coord , ((5.719999999999999f64) . clone ()) . clone () as sp :: Coord ,) , 37u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 38u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 39u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 40u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 41u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 42u32 => (((14.560000000000002f64) . clone ()) . clone () as sp :: Coord , ((14.560000000000002f64) . clone ()) . clone () as sp :: Coord , ((5.719999999999999f64) . clone ()) . clone () as sp :: Coord , ((5.719999999999999f64) . clone ()) . clone () as sp :: Coord ,) , 43u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 44u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((26f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 54u32 => (((18f64) . clone ()) . clone () as sp :: Coord , ((130f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 55u32 => (((((((1f64) . clone ()) as f64) * (((40f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * (((300f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 56u32 => (((((((1f64) . clone ()) as f64) * (((40f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * (((300f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 57u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache_ortho ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord ,) , 58u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache_ortho ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord ,) , 59u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache ()) . apply_pin (_self) . get () [4usize]) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache_ortho ()) . apply_pin (_self) . get () [4usize]) . clone ()) . clone () as sp :: Coord ,) , 60u32 => (((34f64) . clone ()) . clone () as sp :: Coord , ((34f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache ()) . apply_pin (_self) . get () [6usize]) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache_ortho ()) . apply_pin (_self) . get () [6usize]) . clone ()) . clone () as sp :: Coord ,) , 61u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache ()) . apply_pin (_self) . get () [8usize]) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache_ortho ()) . apply_pin (_self) . get () [8usize]) . clone ()) . clone () as sp :: Coord ,) , 62u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache ()) . apply_pin (_self) . get () [10usize]) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache_ortho ()) . apply_pin (_self) . get () [10usize]) . clone ()) . clone () as sp :: Coord ,) , 63u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache ()) . apply_pin (_self) . get () [12usize]) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache_ortho ()) . apply_pin (_self) . get () [12usize]) . clone ()) . clone () as sp :: Coord ,) , 64u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache ()) . apply_pin (_self) . get () [14usize]) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_68_layout_cache_ortho ()) . apply_pin (_self) . get () [14usize]) . clone ()) . clone () as sp :: Coord ,) , 65u32 => (((((((1f64) . clone ()) as f64) * (((40f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * (((300f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 66u32 => (((((((1f64) . clone ()) as f64) * (((40f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * (((300f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 67u32 => (((((((1f64) . clone ()) as f64) * (((40f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * (((300f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 68u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 69u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 70u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 71u32 => (((15.680000000000001f64) . clone ()) . clone () as sp :: Coord , ((15.680000000000001f64) . clone ()) . clone () as sp :: Coord , ((6.159999999999999f64) . clone ()) . clone () as sp :: Coord , ((6.159999999999999f64) . clone ()) . clone () as sp :: Coord ,) , 72u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 73u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 74u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 75u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 76u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 77u32 => (((15.680000000000001f64) . clone ()) . clone () as sp :: Coord , ((15.680000000000001f64) . clone ()) . clone () as sp :: Coord , ((6.159999999999999f64) . clone ()) . clone () as sp :: Coord , ((6.159999999999999f64) . clone ()) . clone () as sp :: Coord ,) , 78u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 79u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 80u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 81u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 82u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 83u32 => (((15.680000000000001f64) . clone ()) . clone () as sp :: Coord , ((15.680000000000001f64) . clone ()) . clone () as sp :: Coord , ((6.159999999999999f64) . clone ()) . clone () as sp :: Coord , ((6.159999999999999f64) . clone ()) . clone () as sp :: Coord ,) , 84u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 85u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 86u32 => (((34f64) . clone ()) . clone () as sp :: Coord , ((34f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 87u32 => (((34f64) . clone ()) . clone () as sp :: Coord , ((34f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 88u32 => (((19.040000000000003f64) . clone ()) . clone () as sp :: Coord , ((19.040000000000003f64) . clone ()) . clone () as sp :: Coord , ((7.479999999999999f64) . clone ()) . clone () as sp :: Coord , ((7.479999999999999f64) . clone ()) . clone () as sp :: Coord ,) , 89u32 => (((19.040000000000003f64) . clone ()) . clone () as sp :: Coord , ((19.040000000000003f64) . clone ()) . clone () as sp :: Coord , ((7.479999999999999f64) . clone ()) . clone () as sp :: Coord , ((7.479999999999999f64) . clone ()) . clone () as sp :: Coord ,) , 90u32 => (((34f64) . clone ()) . clone () as sp :: Coord , ((34f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 91u32 => (((34f64) . clone ()) . clone () as sp :: Coord , ((34f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 92u32 => (((19.040000000000003f64) . clone ()) . clone () as sp :: Coord , ((19.040000000000003f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 93u32 => (((19.040000000000003f64) . clone ()) . clone () as sp :: Coord , ((19.040000000000003f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 94u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 95u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 96u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 97u32 => (((15.680000000000001f64) . clone ()) . clone () as sp :: Coord , ((15.680000000000001f64) . clone ()) . clone () as sp :: Coord , ((6.159999999999999f64) . clone ()) . clone () as sp :: Coord , ((6.159999999999999f64) . clone ()) . clone () as sp :: Coord ,) , 98u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 99u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 100u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 101u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 102u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 103u32 => (((15.680000000000001f64) . clone ()) . clone () as sp :: Coord , ((15.680000000000001f64) . clone ()) . clone () as sp :: Coord , ((6.159999999999999f64) . clone ()) . clone () as sp :: Coord , ((6.159999999999999f64) . clone ()) . clone () as sp :: Coord ,) , 104u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 105u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 106u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 107u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 108u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 109u32 => (((15.680000000000001f64) . clone ()) . clone () as sp :: Coord , ((15.680000000000001f64) . clone ()) . clone () as sp :: Coord , ((6.159999999999999f64) . clone ()) . clone () as sp :: Coord , ((6.159999999999999f64) . clone ()) . clone () as sp :: Coord ,) , 110u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 111u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 112u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 113u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 114u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 115u32 => (((15.680000000000001f64) . clone ()) . clone () as sp :: Coord , ((15.680000000000001f64) . clone ()) . clone () as sp :: Coord , ((6.159999999999999f64) . clone ()) . clone () as sp :: Coord , ((6.159999999999999f64) . clone ()) . clone () as sp :: Coord ,) , 116u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 117u32 => (((28f64) . clone ()) . clone () as sp :: Coord , ((28f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 118u32 => (((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 119u32 => (((40f64) . clone ()) . clone () as sp :: Coord , ((160f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 120u32 => (((((((1f64) . clone ()) as f64) * (((40f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * (((160f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 121u32 => (((((((1f64) . clone ()) as f64) * (((40f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * (((160f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 122u32 => (((16f64) . clone ()) . clone () as sp :: Coord , ((92f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_135_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_135_layout_cache_ortho ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord ,) , 123u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_135_layout_cache_ortho ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , ((36f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_135_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_135_layout_cache_ortho ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord ,) , 124u32 => (((((((1f64) . clone ()) as f64) * (((40f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * (((160f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 125u32 => (((((((1f64) . clone ()) as f64) * (((40f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * (((160f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 126u32 => (((((((1f64) . clone ()) as f64) * (((40f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * (((160f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 129u32 => (((170f64) . clone ()) . clone () as sp :: Coord , ((720f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 130u32 => (((170f64) . clone ()) . clone () as sp :: Coord , ((720f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 131u32 => (((20f64) . clone ()) . clone () as sp :: Coord , ((140f64) . clone ()) . clone () as sp :: Coord , ((16f64) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord ,) , 132u32 => (((20f64) . clone ()) . clone () as sp :: Coord , ((44f64) . clone ()) . clone () as sp :: Coord , ((616f64) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord ,) , 133u32 => (((24f64) . clone ()) . clone () as sp :: Coord , ((24f64) . clone ()) . clone () as sp :: Coord , ((674f64) . clone ()) . clone () as sp :: Coord , ((5f64) . clone ()) . clone () as sp :: Coord ,) , 134u32 => (((116f64) . clone ()) . clone () as sp :: Coord , ((704f64) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord , ((34f64) . clone ()) . clone () as sp :: Coord ,) , 135u32 => (((20f64) . clone ()) . clone () as sp :: Coord , ((44f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 136u32 => (((24f64) . clone ()) . clone () as sp :: Coord , ((24f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 137u32 => (((24f64) . clone ()) . clone () as sp :: Coord , ((24f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 138u32 => (((24f64) . clone ()) . clone () as sp :: Coord , ((24f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 139u32 => (((13.440000000000001f64) . clone ()) . clone () as sp :: Coord , ((13.440000000000001f64) . clone ()) . clone () as sp :: Coord , ((5.279999999999999f64) . clone ()) . clone () as sp :: Coord , ((5.279999999999999f64) . clone ()) . clone () as sp :: Coord ,) , 140u32 => (((24f64) . clone ()) . clone () as sp :: Coord , ((24f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 141u32 => (((24f64) . clone ()) . clone () as sp :: Coord , ((24f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 142u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#flickable_152 () + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#flickable_152 () + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#flickable_152 () + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#flickable_152 () + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 144u32 => (((170f64) . clone ()) . clone () as sp :: Coord , ((720f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 145u32 => (((170f64) . clone ()) . clone () as sp :: Coord , ((720f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 146u32 => (((136f64) . clone ()) . clone () as sp :: Coord , ((340f64) . clone ()) . clone () as sp :: Coord , ((190f64) . clone ()) . clone () as sp :: Coord , ((17f64) . clone ()) . clone () as sp :: Coord ,) , 147u32 => (((136f64) . clone ()) . clone () as sp :: Coord , ((340f64) . clone ()) . clone () as sp :: Coord , ((190f64) . clone ()) . clone () as sp :: Coord , ((17f64) . clone ()) . clone () as sp :: Coord ,) , 148u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , ((304f64) . clone ()) . clone () as sp :: Coord , ((18f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord ,) , 149u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , ((304f64) . clone ()) . clone () as sp :: Coord , ((18f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord ,) , 150u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) . get () [5usize]) . clone ()) . clone () as sp :: Coord , ((304f64) . clone ()) . clone () as sp :: Coord , ((18f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) . get () [4usize]) . clone ()) . clone () as sp :: Coord ,) , 151u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) . get () [7usize]) . clone ()) . clone () as sp :: Coord , ((304f64) . clone ()) . clone () as sp :: Coord , ((18f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) . get () [6usize]) . clone ()) . clone () as sp :: Coord ,) , 152u32 => ((((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) . get () [9usize]) . clone ()) . clone () as sp :: Coord , ((304f64) . clone ()) . clone () as sp :: Coord , ((18f64) . clone ()) . clone () as sp :: Coord , (((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_layout_cache ()) . apply_pin (_self) . get () [8usize]) . clone ()) . clone () as sp :: Coord ,) , 153u32 => (((24f64) . clone ()) . clone () as sp :: Coord , ((24f64) . clone ()) . clone () as sp :: Coord , ((304f64) . clone ()) . clone () as sp :: Coord , ((6f64) . clone ()) . clone () as sp :: Coord ,) , 154u32 => (((24f64) . clone ()) . clone () as sp :: Coord , ((24f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 155u32 => (((24f64) . clone ()) . clone () as sp :: Coord , ((24f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 156u32 => (((24f64) . clone ()) . clone () as sp :: Coord , ((24f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 157u32 => (((13.440000000000001f64) . clone ()) . clone () as sp :: Coord , ((13.440000000000001f64) . clone ()) . clone () as sp :: Coord , ((5.279999999999999f64) . clone ()) . clone () as sp :: Coord , ((5.279999999999999f64) . clone ()) . clone () as sp :: Coord ,) , 158u32 => (((24f64) . clone ()) . clone () as sp :: Coord , ((24f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 159u32 => (((24f64) . clone ()) . clone () as sp :: Coord , ((24f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 45u32 ..= 53u32 => return InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (_self) . item_geometry (index - 45u32 + 1) , 127u32 ..= 128u32 => return InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () . apply_pin (_self) . item_geometry (index - 127u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 8u32 => sp :: r#AccessibleRole :: r#Image , 9u32 => sp :: r#AccessibleRole :: r#Text , 24u32 => sp :: r#AccessibleRole :: r#Image , 30u32 => sp :: r#AccessibleRole :: r#Image , 36u32 => sp :: r#AccessibleRole :: r#Image , 42u32 => sp :: r#AccessibleRole :: r#Image , 54u32 => sp :: r#AccessibleRole :: r#Text , 71u32 => sp :: r#AccessibleRole :: r#Image , 77u32 => sp :: r#AccessibleRole :: r#Image , 83u32 => sp :: r#AccessibleRole :: r#Image , 92u32 => sp :: r#AccessibleRole :: r#Image , 93u32 => sp :: r#AccessibleRole :: r#Image , 97u32 => sp :: r#AccessibleRole :: r#Image , 103u32 => sp :: r#AccessibleRole :: r#Image , 109u32 => sp :: r#AccessibleRole :: r#Image , 115u32 => sp :: r#AccessibleRole :: r#Image , 123u32 => sp :: r#AccessibleRole :: r#Text , 131u32 => sp :: r#AccessibleRole :: r#Text , 132u32 => sp :: r#AccessibleRole :: r#Text , 139u32 => sp :: r#AccessibleRole :: r#Image , 148u32 => sp :: r#AccessibleRole :: r#Text , 149u32 => sp :: r#AccessibleRole :: r#Text , 150u32 => sp :: r#AccessibleRole :: r#Text , 151u32 => sp :: r#AccessibleRole :: r#Text , 152u32 => sp :: r#AccessibleRole :: r#Text , 157u32 => sp :: r#AccessibleRole :: r#Image , 14u32 => InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (_self) . accessible_role (0) , 45u32 ..= 53u32 => InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (_self) . accessible_role (index - 45u32 + 1) , 122u32 => InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () . apply_pin (_self) . accessible_role (0) , 127u32 ..= 128u32 => InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () . apply_pin (_self) . accessible_role (index - 127u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (9u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerMainWindow :: FIELD_OFFSETS . r#text_30 () + sp :: r#ComplexText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . get ()) , (54u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ({
                     * & InnerUIState :: FIELD_OFFSETS . r#mode_text () }
                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) , (123u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ({
                     * & InnerUIState :: FIELD_OFFSETS . r#volume_text () }
                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get ()) , (131u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("播放列表")) , (132u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("清空")) , (148u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("关于 zzhMusicPlayer")) , (149u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("开发者：zzhzhouzhou")) , (150u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("简单、轻量、纯粹的音频播放器")) , (151u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("技术：Rust · Slint · rodio · symphonia")) , (152u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("版本：1.0.0")) , (14u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (_self) . accessible_string_property (0 , what) , (45u32 ..= 53u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (_self) . accessible_string_property (index - 45u32 + 1 , what) , (122u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () . apply_pin (_self) . accessible_string_property (0 , what) , (127u32 ..= 128u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () . apply_pin (_self) . accessible_string_property (index - 127u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (14u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (_self) . accessibility_action (0 , action) , (45u32 ..= 53u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (_self) . accessibility_action (index - 45u32 + 1 , action) , (122u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () . apply_pin (_self) . accessibility_action (0 , action) , (127u32 ..= 128u32 , _) => InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () . apply_pin (_self) . accessibility_action (index - 127u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 14u32 => InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (_self) . supported_accessibility_actions (0) , 45u32 ..= 53u32 => InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (_self) . supported_accessibility_actions (index - 45u32 + 1) , 122u32 => InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () . apply_pin (_self) . supported_accessibility_actions (0) , 127u32 ..= 128u32 => InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () . apply_pin (_self) . supported_accessibility_actions (index - 127u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 45u32 ..= 53u32 => InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () . apply_pin (_self) . item_element_infos (index - 45u32 + 1) , 127u32 ..= 128u32 => InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () . apply_pin (_self) . item_element_infos (index - 127u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_clear_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1) , false , sp :: FocusReason :: Programmatic)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_control_bar_62_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ((((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = (0f64) . clone () as _ ;
                 the_struct . r#stretch = (1f64) . clone () as _ ;
                 the_struct }
            ) . clone ())) + (((_self . r#fn_empty_68_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone ())))) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_empty_162_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_163 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , args . 0 . clone () as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 148u32 - 1))) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_164 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , args . 0 . clone () as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 149u32 - 1))) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_165 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , args . 0 . clone () as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 150u32 - 1))) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_166 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , args . 0 . clone () as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 151u32 - 1))) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#text_167 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , args . 0 . clone () as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 152u32 - 1))) . clone () as _ ;
                 the_struct }
            ) . clone ()]) as _ , (InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_spacing ()) . apply_pin (_self) . get () . get () as _ , & {
                 let mut the_struct = sp :: Padding :: default () ;
                 the_struct . r#begin = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_padding_top ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct . r#end = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_162_padding_bottom ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct }
             as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_empty_68_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ({
                     let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_71 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , args . 0 . clone () as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 69u32 - 1))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_71_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_71_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                ) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ({
                     let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_78 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , args . 0 . clone () as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 75u32 - 1))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_78_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_78_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                ) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ({
                     let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_85 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , args . 0 . clone () as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 81u32 - 1))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_85_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_85_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                ) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ({
                     let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_91 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , args . 0 . clone () as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 86u32 - 1))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_91_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_91_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                ) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ({
                     let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_101 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , args . 0 . clone () as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 95u32 - 1))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_101_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_101_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                ) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ({
                     let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_108 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , args . 0 . clone () as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 101u32 - 1))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_108_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_108_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                ) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ({
                     let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_115 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , args . 0 . clone () as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 107u32 - 1))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_115_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_115_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                ) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ({
                     let r#layout_info = (sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#rectangle_122 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , args . 0 . clone () as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 113u32 - 1))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_122_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_rectangle_122_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                ) . clone () as _ ;
                 the_struct }
            ) . clone ()]) as _ , & {
                 let mut the_struct = sp :: Padding :: default () ;
                 the_struct . r#begin = (0f64) . clone () as _ ;
                 the_struct . r#end = (0f64) . clone () as _ ;
                 the_struct }
             as _)) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1) , true , sp :: FocusReason :: Programmatic)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             (((((sp :: Item :: layout_info ((InnerMainWindow :: FIELD_OFFSETS . r#root_21 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone ())) + (((_self . r#fn_rectangle_27_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone ())))) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_rectangle_161_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ((((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = (0f64) . clone () as _ ;
                 the_struct . r#stretch = (1f64) . clone () as _ ;
                 the_struct }
            ) . clone ())) + (((_self . r#fn_empty_162_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone ())))) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_rectangle_27_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ((((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = (0f64) . clone () as _ ;
                 the_struct . r#stretch = (1f64) . clone () as _ ;
                 the_struct }
            ) . clone ())) + ((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = ((InnerMainWindow :: FIELD_OFFSETS . r#root_21_image_29_preferred_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct . r#stretch = (0f64) . clone () as _ ;
                 the_struct }
            ) . clone ())))) as _ }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_playlistrow_155 {
         r#playlistrow_155 : InnerPlaylistRow_root_16 , r#model_data : sp :: Property < sp :: SharedString > , r#model_index : sp :: Property < i32 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_playlistrow_155 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_playlistrow_155 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerPlaylistRow_root_16 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_play_at () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#play_at () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& (((InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get ()) . clone () as _ ,)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_index ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_remove ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . callback_tracker_remove_track () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . get () ;
                                 {
                                     * & InnerUIState :: FIELD_OFFSETS . r#remove_track () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_UIState . as_ref ()) . call (& (((InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get ()) . clone () as _ ,)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#model_data ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#flickable_152 () + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_21_empty_154_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [0usize] as usize) + (InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get () as usize * 2usize) . unwrap_or (& (0 as _)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerPlaylistRow_root_16 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#playlistrow_155 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#playlistrow_155 . tree_index . get ())) , sp :: Orientation :: Vertical => {
                     let r#layout_info = (sp :: Item :: layout_info ((InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#playlistrow_155 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#playlistrow_155 . tree_index . get ()))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (26f64) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (26f64) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             (InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . get () as usize }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((26f64) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#flickable_152 () + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , (((InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 1u32 ..= 4u32 => return InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () . apply_pin (_self) . accessible_role (0) , 1u32 ..= 4u32 => InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , _) => InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 4u32 , _) => InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , _) => InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 4u32 , _) => InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 4u32 => InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 4u32 => InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_playlistrow_155 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             5usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 1u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 3u32 , item_array_index : 4u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_playlistrow_155 , sp :: ItemVTable , sp :: AllowPin > ;
             5usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#root_16 ()) , sp :: VOffset :: new (InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#text_17 ()) , sp :: VOffset :: new (InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#toucharea_18 ()) , sp :: VOffset :: new (InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#image_19 ()) , sp :: VOffset :: new (InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#playlistrow_155 () + InnerPlaylistRow_root_16 :: FIELD_OFFSETS . r#toucharea_20 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_playlistrow_155) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_playlistrow_155 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_playlistrow_155 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_playlistrow_155 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_playlistrow_155 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 143u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             self . ensure_instantiated () }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_playlistrow_155 {
         type Data = sp :: SharedString ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             (InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#model_index ()) . apply_pin (_self) . set (_index as _) ;
             (InnerComponent_playlistrow_155 :: FIELD_OFFSETS . r#model_data ()) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_playlistrow_155 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerMainWindow {
         fn new () -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = SharedGlobals :: new (sp :: VRc :: downgrade (& self_dyn_rc)) ;
             let _ = sp :: VRc :: map (self_rc . clone () , | x | x) . as_pin_ref () . globals . set (globals . clone ()) ;
             globals . init_globals () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             160usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 7u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 13u32 , children_index : 8u32 , parent_index : 3u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 21u32 , parent_index : 7u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 21u32 , parent_index : 7u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 21u32 , parent_index : 7u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 27u32 , parent_index : 7u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 33u32 , parent_index : 7u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 39u32 , parent_index : 7u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 6u32 , children_index : 45u32 , parent_index : 7u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 54u32 , parent_index : 7u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 10u32 , children_index : 55u32 , parent_index : 7u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 118u32 , parent_index : 7u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 129u32 , parent_index : 7u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 131u32 , parent_index : 7u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 144u32 , parent_index : 7u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 22u32 , parent_index : 10u32 , item_array_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 23u32 , parent_index : 21u32 , item_array_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 26u32 , parent_index : 22u32 , item_array_index : 23u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 27u32 , parent_index : 22u32 , item_array_index : 24u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 27u32 , parent_index : 22u32 , item_array_index : 25u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 27u32 , parent_index : 23u32 , item_array_index : 26u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 28u32 , parent_index : 11u32 , item_array_index : 27u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 29u32 , parent_index : 27u32 , item_array_index : 28u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 32u32 , parent_index : 28u32 , item_array_index : 29u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 33u32 , parent_index : 28u32 , item_array_index : 30u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 33u32 , parent_index : 28u32 , item_array_index : 31u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 33u32 , parent_index : 29u32 , item_array_index : 32u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 34u32 , parent_index : 12u32 , item_array_index : 33u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 35u32 , parent_index : 33u32 , item_array_index : 34u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 38u32 , parent_index : 34u32 , item_array_index : 35u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 39u32 , parent_index : 34u32 , item_array_index : 36u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 39u32 , parent_index : 34u32 , item_array_index : 37u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 39u32 , parent_index : 35u32 , item_array_index : 38u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 40u32 , parent_index : 13u32 , item_array_index : 39u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 41u32 , parent_index : 39u32 , item_array_index : 40u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 44u32 , parent_index : 40u32 , item_array_index : 41u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 45u32 , parent_index : 40u32 , item_array_index : 42u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 45u32 , parent_index : 40u32 , item_array_index : 43u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 45u32 , parent_index : 41u32 , item_array_index : 44u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 51u32 , parent_index : 14u32 , item_array_index : 45u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 51u32 , parent_index : 14u32 , item_array_index : 46u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 53u32 , parent_index : 14u32 , item_array_index : 47u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 54u32 , parent_index : 14u32 , item_array_index : 48u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 54u32 , parent_index : 14u32 , item_array_index : 49u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 52u32 , parent_index : 46u32 , item_array_index : 50u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 53u32 , parent_index : 51u32 , item_array_index : 51u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 54u32 , parent_index : 47u32 , item_array_index : 52u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 55u32 , parent_index : 15u32 , item_array_index : 53u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 65u32 , parent_index : 16u32 , item_array_index : 54u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 67u32 , parent_index : 16u32 , item_array_index : 55u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 68u32 , parent_index : 16u32 , item_array_index : 56u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 74u32 , parent_index : 16u32 , item_array_index : 57u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 80u32 , parent_index : 16u32 , item_array_index : 58u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 86u32 , parent_index : 16u32 , item_array_index : 59u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 94u32 , parent_index : 16u32 , item_array_index : 60u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 100u32 , parent_index : 16u32 , item_array_index : 61u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 106u32 , parent_index : 16u32 , item_array_index : 62u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 112u32 , parent_index : 16u32 , item_array_index : 63u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 67u32 , parent_index : 55u32 , item_array_index : 64u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 67u32 , parent_index : 55u32 , item_array_index : 65u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 68u32 , parent_index : 56u32 , item_array_index : 66u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 69u32 , parent_index : 57u32 , item_array_index : 67u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 70u32 , parent_index : 68u32 , item_array_index : 68u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 73u32 , parent_index : 69u32 , item_array_index : 69u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 74u32 , parent_index : 69u32 , item_array_index : 70u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 74u32 , parent_index : 69u32 , item_array_index : 71u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 74u32 , parent_index : 70u32 , item_array_index : 72u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 75u32 , parent_index : 58u32 , item_array_index : 73u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 76u32 , parent_index : 74u32 , item_array_index : 74u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 79u32 , parent_index : 75u32 , item_array_index : 75u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 80u32 , parent_index : 75u32 , item_array_index : 76u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 80u32 , parent_index : 75u32 , item_array_index : 77u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 80u32 , parent_index : 76u32 , item_array_index : 78u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 81u32 , parent_index : 59u32 , item_array_index : 79u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 82u32 , parent_index : 80u32 , item_array_index : 80u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 85u32 , parent_index : 81u32 , item_array_index : 81u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 86u32 , parent_index : 81u32 , item_array_index : 82u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 86u32 , parent_index : 81u32 , item_array_index : 83u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 86u32 , parent_index : 82u32 , item_array_index : 84u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 87u32 , parent_index : 60u32 , item_array_index : 85u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 91u32 , parent_index : 86u32 , item_array_index : 86u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 92u32 , parent_index : 86u32 , item_array_index : 87u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 93u32 , parent_index : 86u32 , item_array_index : 88u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 94u32 , parent_index : 86u32 , item_array_index : 89u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 92u32 , parent_index : 87u32 , item_array_index : 90u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 93u32 , parent_index : 88u32 , item_array_index : 91u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 94u32 , parent_index : 89u32 , item_array_index : 92u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 95u32 , parent_index : 61u32 , item_array_index : 93u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 96u32 , parent_index : 94u32 , item_array_index : 94u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 99u32 , parent_index : 95u32 , item_array_index : 95u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 100u32 , parent_index : 95u32 , item_array_index : 96u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 100u32 , parent_index : 95u32 , item_array_index : 97u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 100u32 , parent_index : 96u32 , item_array_index : 98u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 101u32 , parent_index : 62u32 , item_array_index : 99u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 102u32 , parent_index : 100u32 , item_array_index : 100u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 105u32 , parent_index : 101u32 , item_array_index : 101u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 106u32 , parent_index : 101u32 , item_array_index : 102u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 106u32 , parent_index : 101u32 , item_array_index : 103u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 106u32 , parent_index : 102u32 , item_array_index : 104u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 107u32 , parent_index : 63u32 , item_array_index : 105u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 108u32 , parent_index : 106u32 , item_array_index : 106u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 111u32 , parent_index : 107u32 , item_array_index : 107u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 112u32 , parent_index : 107u32 , item_array_index : 108u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 112u32 , parent_index : 107u32 , item_array_index : 109u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 112u32 , parent_index : 108u32 , item_array_index : 110u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 113u32 , parent_index : 64u32 , item_array_index : 111u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 114u32 , parent_index : 112u32 , item_array_index : 112u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 117u32 , parent_index : 113u32 , item_array_index : 113u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 118u32 , parent_index : 113u32 , item_array_index : 114u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 118u32 , parent_index : 113u32 , item_array_index : 115u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 118u32 , parent_index : 114u32 , item_array_index : 116u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 119u32 , parent_index : 17u32 , item_array_index : 117u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 120u32 , parent_index : 118u32 , item_array_index : 118u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 124u32 , parent_index : 119u32 , item_array_index : 119u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 126u32 , parent_index : 119u32 , item_array_index : 120u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 127u32 , parent_index : 119u32 , item_array_index : 121u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 129u32 , parent_index : 119u32 , item_array_index : 122u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 126u32 , parent_index : 120u32 , item_array_index : 123u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 126u32 , parent_index : 120u32 , item_array_index : 124u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 127u32 , parent_index : 121u32 , item_array_index : 125u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 129u32 , parent_index : 122u32 , item_array_index : 126u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 129u32 , parent_index : 122u32 , item_array_index : 127u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 130u32 , parent_index : 18u32 , item_array_index : 128u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 131u32 , parent_index : 129u32 , item_array_index : 129u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 135u32 , parent_index : 19u32 , item_array_index : 130u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 1u32 , children_index : 135u32 , parent_index : 19u32 , item_array_index : 131u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 136u32 , parent_index : 19u32 , item_array_index : 132u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 142u32 , parent_index : 19u32 , item_array_index : 133u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 136u32 , parent_index : 132u32 , item_array_index : 134u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 137u32 , parent_index : 133u32 , item_array_index : 135u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 138u32 , parent_index : 136u32 , item_array_index : 136u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 141u32 , parent_index : 137u32 , item_array_index : 137u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 142u32 , parent_index : 137u32 , item_array_index : 138u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 142u32 , parent_index : 137u32 , item_array_index : 139u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 142u32 , parent_index : 138u32 , item_array_index : 140u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 143u32 , parent_index : 134u32 , item_array_index : 141u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 142u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 145u32 , parent_index : 20u32 , item_array_index : 142u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 148u32 , parent_index : 144u32 , item_array_index : 143u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 148u32 , parent_index : 144u32 , item_array_index : 144u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 6u32 , children_index : 148u32 , parent_index : 144u32 , item_array_index : 145u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 154u32 , parent_index : 147u32 , item_array_index : 146u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 154u32 , parent_index : 147u32 , item_array_index : 147u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 154u32 , parent_index : 147u32 , item_array_index : 148u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 154u32 , parent_index : 147u32 , item_array_index : 149u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 154u32 , parent_index : 147u32 , item_array_index : 150u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 154u32 , parent_index : 147u32 , item_array_index : 151u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 155u32 , parent_index : 153u32 , item_array_index : 152u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 156u32 , parent_index : 154u32 , item_array_index : 153u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 159u32 , parent_index : 155u32 , item_array_index : 154u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 160u32 , parent_index : 155u32 , item_array_index : 155u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 160u32 , parent_index : 155u32 , item_array_index : 156u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 160u32 , parent_index : 156u32 , item_array_index : 157u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerMainWindow , sp :: ItemVTable , sp :: AllowPin > ;
             158usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#root_21 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#shortcuts_22 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_26 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_27 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#keybinding_23 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#keybinding_24 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#keybinding_25 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_clip_28 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_29 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#text_30 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_31 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_38 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_45 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_52 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#root_1 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_60 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#control_bar_62 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_127 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_138 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_141 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_157 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_32 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_33 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_34 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_36 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#ta_37 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_35 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_39 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_40 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_41 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_43 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#ta_44 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_42 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_46 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_47 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_48 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_50 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#ta_51 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_49 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_53 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_54 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_55 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_57 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#ta_58 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_56 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_2 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#rectangle_3 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#_Opacity_6 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#rectangle_8 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#toucharea_12 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#_clip_4 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#image_5 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#waveformarea_59 () + InnerWaveformArea_root_1 :: FIELD_OFFSETS . r#rectangle_7 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#text_61 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_63 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_66 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_69 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_76 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_83 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_90 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_99 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_106 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_113 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_120 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_64 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_65 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_67 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_70 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_71 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_72 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_74 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#ta_75 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_73 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_77 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_78 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_79 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_81 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#ta_82 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_80 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_84 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_85 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_86 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_88 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#ta_89 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_87 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_91 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_92 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_94 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_96 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#ta_98 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_93 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_95 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_97 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_100 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_101 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_102 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_104 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#ta_105 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_103 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_107 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_108 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_109 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_111 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#ta_112 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_110 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_114 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_115 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_116 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_118 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#ta_119 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_117 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_121 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_122 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_123 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_125 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#ta_126 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_124 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_visibility_128 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_129 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_130 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_133 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#root_13 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#text_137 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_131 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_132 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_134 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#rectangle_14 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#volumebar_136 () + InnerVolumeBar_root_13 :: FIELD_OFFSETS . r#toucharea_15 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_139 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_140 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#text_142 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#text_143 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_145 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#flickable_152 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_144 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_146 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_147 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_148 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_150 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#ta_151 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_149 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_viewport_153 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_158 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#toucharea_159 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_shadow_160 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_161 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#text_163 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#text_164 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#text_165 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#text_166 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#text_167 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_168 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Transform_169 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_170 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#_Opacity_171 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#image_173 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#ta_174 ()) , sp :: VOffset :: new (InnerMainWindow :: FIELD_OFFSETS . r#rectangle_172 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerMainWindow) ;
         }
     ;
     impl sp :: PinnedDrop for InnerMainWindow {
         fn drop (self : :: core :: pin :: Pin < & mut InnerMainWindow >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerMainWindow {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerMainWindow > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             self . ensure_instantiated () }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#MainWindow (sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow >) ;
     impl r#MainWindow {
         pub fn new () -> :: core :: result :: Result < Self , slint :: PlatformError > {
             slint :: private_unstable_api :: ensure_backend () ? ;
             let inner = InnerMainWindow :: new () ? ;
             inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             InnerMainWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             let window = inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             sp :: WindowInner :: from_pub (window . window ()) . ensure_tree_instantiated () ;
             :: core :: result :: Result :: Ok (Self (inner)) }
         # [cfg (false)] pub fn new_with_context (ctx : sp :: SlintContext) -> :: core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerMainWindow :: new () ? ;
             inner . globals . get () . unwrap () . create_window_from_context (ctx) ? ;
             InnerMainWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             let window = inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             sp :: WindowInner :: from_pub (window . window ()) . ensure_tree_instantiated () ;
             :: core :: result :: Result :: Ok (Self (inner)) }
         # [cfg (false)] pub fn new_with_existing_window (window : & slint :: Window) -> :: core :: result :: Result < Self , slint :: PlatformError > {
             slint :: private_unstable_api :: ensure_backend () ? ;
             let inner = InnerMainWindow :: new () ? ;
             inner . globals . get () . unwrap () . create_window_from_existing (window) ? ;
             InnerMainWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             let window = inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             sp :: WindowInner :: from_pub (window . window ()) . ensure_tree_instantiated () ;
             :: core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn invoke_clear_focus (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             _self . r#fn_clear_focus () }
         # [allow (dead_code)] pub fn invoke_focus (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             _self . r#fn_focus () }
         # [allow (dead_code)] fn get_progress_frac (& self , _private_property : ()) {
             }
         # [allow (dead_code)] fn set_progress_frac (& self , _private_property : ()) {
             }
         }
     impl From < r#MainWindow > for sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow > {
         fn from (value : r#MainWindow) -> Self {
             value . 0 }
         }
     impl slint :: StrongHandle for r#MainWindow {
         type WeakInner = sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow > ;
         fn upgrade_from_weak_inner (inner : & Self :: WeakInner) -> sp :: Option < Self > {
             sp :: Some (Self (inner . upgrade () ?)) }
         }
     impl slint :: ComponentHandle for r#MainWindow {
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (sp :: VRc :: downgrade (& self . 0)) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         fn run (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             sp :: WindowInner :: from_pub (self . window ()) . context () . run_event_loop () ? ;
             self . hide () ? ;
             :: core :: result :: Result :: Ok (()) }
         fn show (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () . unwrap () . window () }
         }
     struct SharedGlobals {
         global_UIState : :: core :: pin :: Pin < sp :: Rc < InnerUIState >> , window_adapter : sp :: OnceCell < sp :: WindowAdapterRc > , root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable > , }
     impl SharedGlobals {
         fn new (root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable >) -> sp :: Rc < Self > {
             sp :: Rc :: new (Self {
                 global_UIState : InnerUIState :: new () , window_adapter : :: core :: default :: Default :: default () , root_item_tree_weak , }
            ) }
         fn init_globals (self : & sp :: Rc < Self >) {
             self . global_UIState . clone () . init (self) ;
             }
         # [allow (dead_code)] fn clone_with_window_adapter (& self , window_adapter : sp :: WindowAdapterRc) -> sp :: Rc < Self > {
             sp :: Rc :: new (Self {
                 global_UIState : self . global_UIState . clone () , window_adapter : window_adapter . into () , root_item_tree_weak : :: core :: default :: Default :: default () , }
            ) }
         fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             sp :: Rc :: clone (self . window_adapter_ref () . unwrap ()) }
         fn window_adapter_ref (& self) -> sp :: Result < & sp :: Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let root_rc = self . root_item_tree_weak . upgrade () . unwrap () ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& root_rc) ;
                 :: core :: result :: Result :: Ok (adapter) }
            ) }
         # [cfg (false)] fn create_window_from_context (& self , ctx : sp :: SlintContext) -> sp :: Result < () , slint :: PlatformError > {
             let adapter = ctx . platform () . create_window_adapter () ? ;
             sp :: WindowInner :: from_pub (adapter . window ()) . set_context (ctx) ;
             let root_rc = self . root_item_tree_weak . upgrade () . unwrap () ;
             sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& root_rc) ;
             self . window_adapter . set (adapter) . map_err (| _ | ()) . expect ("The window shouldn't be initialized before this call") ;
             sp :: Ok (()) }
         # [cfg (false)] fn create_window_from_existing (& self , window : & slint :: Window) -> sp :: Result < () , slint :: PlatformError > {
             let adapter = sp :: WindowInner :: from_pub (window) . window_adapter () ;
             let root_rc = self . root_item_tree_weak . upgrade () . unwrap () ;
             sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& root_rc) ;
             self . window_adapter . set (adapter) . map_err (| _ | ()) . expect ("The window shouldn't be initialized before this call") ;
             sp :: Ok (()) }
         fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter . get () . cloned () }
         }
     static SLINT_EMBEDDED_RESOURCE_0 : & 'static [u8] = :: core :: include_bytes ! ("E:\\zzhMusicPlayer\\icons\\删除.svg") ;
     static SLINT_EMBEDDED_RESOURCE_1 : & 'static [u8] = :: core :: include_bytes ! ("E:\\zzhMusicPlayer\\icons\\下一首.svg") ;
     static SLINT_EMBEDDED_RESOURCE_2 : & 'static [u8] = :: core :: include_bytes ! ("E:\\zzhMusicPlayer\\icons\\快进.svg") ;
     static SLINT_EMBEDDED_RESOURCE_3 : & 'static [u8] = :: core :: include_bytes ! ("E:\\zzhMusicPlayer\\icons\\音量.svg") ;
     static SLINT_EMBEDDED_RESOURCE_4 : & 'static [u8] = :: core :: include_bytes ! ("E:\\zzhMusicPlayer\\icons\\播放列表.svg") ;
     static SLINT_EMBEDDED_RESOURCE_5 : & 'static [u8] = :: core :: include_bytes ! ("E:\\zzhMusicPlayer\\icons\\关闭.svg") ;
     static SLINT_EMBEDDED_RESOURCE_6 : & 'static [u8] = :: core :: include_bytes ! ("E:\\zzhMusicPlayer\\icons\\信息圆.svg") ;
     static SLINT_EMBEDDED_RESOURCE_7 : & 'static [u8] = :: core :: include_bytes ! ("E:\\zzhMusicPlayer\\icons\\最小化.svg") ;
     static SLINT_EMBEDDED_RESOURCE_8 : & 'static [u8] = :: core :: include_bytes ! ("E:\\zzhMusicPlayer\\icons\\置顶.svg") ;
     static SLINT_EMBEDDED_RESOURCE_9 : & 'static [u8] = :: core :: include_bytes ! ("E:\\zzhMusicPlayer\\icons\\模式.svg") ;
     static SLINT_EMBEDDED_RESOURCE_10 : & 'static [u8] = :: core :: include_bytes ! ("E:\\zzhMusicPlayer\\icons\\快退.svg") ;
     static SLINT_EMBEDDED_RESOURCE_11 : & 'static [u8] = :: core :: include_bytes ! ("E:\\zzhMusicPlayer\\icons\\上一首.svg") ;
     static SLINT_EMBEDDED_RESOURCE_12 : & 'static [u8] = :: core :: include_bytes ! ("E:\\zzhMusicPlayer\\icons\\播放中.svg") ;
     static SLINT_EMBEDDED_RESOURCE_13 : & 'static [u8] = :: core :: include_bytes ! ("E:\\zzhMusicPlayer\\icons\\暂停.svg") ;
     }
 # [allow (unused_imports)] pub use slint_generatedMainWindow :: {
     r#MainWindow , r#UIState , }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
