use material_yew::{top_app_bar::MatTopAppBarTitle, MatButton, MatTab, MatTabBar, MatTopAppBar};
use yew::prelude::*;
use material_yew::MatFormfield;
use material_yew::MatTextField;
use material_yew::text_inputs::TextFieldType;

pub struct Model {
    signin: bool,
    link: ComponentLink<Self>,
}

pub enum ModelMsg {
    SignIn,
    SignUp,
}

impl Component for Model {
    type Properties = ();
    type Message = ModelMsg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { signin: true, link }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ModelMsg::SignIn => {
                self.signin = true;
                true
            }
            ModelMsg::SignUp => {
                self.signin = false;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <MatTopAppBar center_title=true>
                    <MatTopAppBarTitle>{ "Blogg" }</MatTopAppBarTitle>

                    <div>
                        <MatTabBar onactivated=self.link.callback(|i| if i == 0 {ModelMsg::SignIn} else {ModelMsg::SignUp} )>
                            <MatTab label="Sign In" />
                            <MatTab label="Sign Up" />
                        </MatTabBar>
                        { self.main() }
                    </div>
                </MatTopAppBar>
            </div>
        }
    }
}

impl Model {
    fn main(&self) -> Html {
        if self.signin {
            html! {
                <div>
                    <div class="centered-content">

                        <p>{ "Welcome Back." }</p>
                        
                        <MatTextField placeholder="username" field_type=TextFieldType::Text />
                        
                        <MatTextField placeholder="password" field_type=TextFieldType::Password />
                        
                    </div>

                    <div class="bottom-buttons-right">
                        <MatButton raised=true outlined=true label="Sign In" />
                    </div>
                </div>
            }
        } else {
            html! { 
                <div>
                    <div class="centered-content">
                        
                        <p>{ "Welcome. Sign Up to create your own blog."}</p>
                        
                        
                        <MatTextField placeholder="username" field_type=TextFieldType::Text />
                        
                        
                        <MatTextField placeholder="password" field_type=TextFieldType::Password />
                         
                    </div>
                    <div class="bottom-buttons-right">
                        <MatButton raised=true outlined=true label="Sign Up" />
                    </div>
                </div>
            }
        }
    }
}
