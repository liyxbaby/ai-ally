export enum Device {
    CPU = "CPU",
    GPU = "GPU",
    Metal = "Metal"
}

export enum PromptTemplate {
    Default = "Default",
    Llama2 = "Llama2",
    Mistral = "Mistral"
}

export interface ConfigInterface {
    device: D